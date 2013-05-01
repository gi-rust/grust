/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
 *
 * Parts based on code from GLib authored by Ryan Lortie
 * (commit 92974b80fc10f494b33ed6760b5417bbbbb83473)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301  USA
 */

#define GLIB_VERSION_MIN_REQUIRED GLIB_VERSION_2_32
#define GLIB_VERSION_MAX_ALLOWED  GLIB_VERSION_CUR_STABLE

#define G_LOG_DOMAIN "Grust"

#include <glib.h>

#define CALL_MINDER_TIMEOUT (500 * G_TIME_SPAN_MILLISECOND)

typedef void (*RustFunc) (gpointer param, GMainContext *context);

typedef enum
{
  RUST_CALL_PENDING,
  RUST_CALL_RETURNED,
  RUST_CALL_TIMED_OUT
} RustCallStatus;

typedef struct _RustCallData RustCallData;
struct _RustCallData
{
  RustFunc func;
  gpointer param;
  GMainContext *context;
  GSource *source;
  gint64 minder_backoff;
  GCond return_cond;
  gint ref_count;
  volatile RustCallStatus status;
};

static void
call_data_unref (RustCallData *call_data)
{
  if (g_atomic_int_dec_and_test (&call_data->ref_count))
    {
      g_cond_clear (&call_data->return_cond);
      g_source_unref (call_data->source);
      g_main_context_unref (call_data->context);
      g_slice_free (RustCallData, call_data);
    }
}

static GPrivate rust_thread_context =
    G_PRIVATE_INIT((GDestroyNotify) g_main_context_unref);

/* These could be per-call and per-context mutexes.
 * The balance of reduced contention vs. extra init/cleanup calls,
 * as well as bookkeeping of the extra context data,
 * would need to be profiled. */
static GMutex call_mutex;
static GMutex rust_context_mutex;
static GCond  rust_context_released_cond;

static gboolean
loop_callback (gpointer data)
{
  RustCallData *call_data = data;

  g_mutex_lock (&call_mutex);
  if (G_LIKELY (call_data->status == RUST_CALL_PENDING))
    {
      g_mutex_unlock (&call_mutex);

      call_data->func (call_data->param, call_data->context);

      g_mutex_lock (&call_mutex);
      call_data->status = RUST_CALL_RETURNED;
      g_cond_broadcast (&call_data->return_cond);
    }
  else
    {
      g_debug ("off-stack call dispatch on an expired call (status=%d)",
               (int) call_data->status);
    }
  g_mutex_unlock (&call_mutex);

  call_data_unref (call_data);

  return FALSE;
}

static void
call_minder (gpointer data, G_GNUC_UNUSED gpointer pool_data)
{
  RustCallData *call_data = data;
  GMainContext *context = call_data->context;
  RustCallStatus status = RUST_CALL_PENDING;
  gint64 end_time;

  end_time = g_get_monotonic_time() + CALL_MINDER_TIMEOUT;

  do
    {
      if (g_main_context_acquire (context))
        {
          status = call_data->status;  /* No locking needed */

          if (status == RUST_CALL_PENDING)
            {
              /* Nothing has been there to drive our call, let's do it now */

              g_source_destroy (call_data->source);

              g_main_context_push_thread_default (context);

              call_data->func (call_data->param, context);

              g_main_context_pop_thread_default (context);

              g_mutex_lock (&call_mutex);
              status = RUST_CALL_RETURNED;
              call_data->status = status;
              g_cond_signal (&call_data->return_cond);
              g_mutex_unlock (&call_mutex);
            }

          g_main_context_release (context);

          /* Unblock a potentially waiting
           * grustna_main_loop_run_thread_local() */
          g_cond_broadcast (&rust_context_released_cond);
        }
      else
        {
          gint64 wakeup_time;

          g_mutex_lock (&call_mutex);

          wakeup_time = g_get_monotonic_time () + call_data->minder_backoff;
          if (wakeup_time > end_time)
            {
              g_critical ("call timed out waiting on context %p", call_data->context);
              status = RUST_CALL_TIMED_OUT;
              call_data->status = status;
              g_cond_signal (&call_data->return_cond);
            }
          else
            {
              if (!g_cond_wait_until (&call_data->return_cond, &call_mutex,
                                         wakeup_time))
                call_data->minder_backoff *= 2;
              status = call_data->status;
            }

          g_mutex_unlock (&call_mutex);
        }
    }
  while (status == RUST_CALL_PENDING);

  call_data_unref (call_data);
}

static gpointer
create_call_minder_pool ()
{
  g_message ("Taking an inefficient, lock-prone call path"
             " -- consider against migrating object references between tasks");

  return g_thread_pool_new (call_minder, NULL,
#if GLIB_CHECK_VERSION(2, 36, 0)
                            g_get_num_processors (),
#else
                            12,
#endif
                            FALSE,
                            NULL);
}

static void
add_call_minder (RustCallData *call_data)
{
  static GOnce pool_once = G_ONCE_INIT;

  GThreadPool *pool;

  g_once (&pool_once, create_call_minder_pool, NULL);
  pool = pool_once.retval;

  g_thread_pool_push (pool, call_data, NULL);
}

static GMainContext *
get_rust_thread_context ()
{
  GMainContext *context;

  context = g_private_get (&rust_thread_context);
  if (context == NULL)
    {
      context = g_main_context_new ();
      g_private_set (&rust_thread_context, context);
    }
  return context;
}

gboolean
grustna_call (RustFunc func, gpointer data, GMainContext *context)
{
  gboolean thread_default_context = FALSE;

  g_return_val_if_fail (func != NULL, FALSE);

  if (context == NULL)
    {
      context = g_main_context_get_thread_default ();
      if (context == NULL)
        context = get_rust_thread_context ();
      else
        thread_default_context = TRUE;
    }

  /* This code is based on g_main_context_invoke_full() */

  if (g_main_context_is_owner (context))
    {
      /* Fastest path: the caller is in the same thread where some code
       * is supposedly driving the loop context affine to this call. */
      func (data, context);
      return TRUE;
    }

  if (g_main_context_acquire (context))
    {
      /* Here, we get to exclusively use the desired loop context
       * that is not (yet) driven by an event loop.
       * This is perfectly OK for non-async functions on objects affine
       * to this context, and matches the behavior of GIO-style async calls
       * that rely on the thread-default context to be eventually driven
       * in order to complete. */

      if (!thread_default_context)
        g_main_context_push_thread_default (context);

      func (data, context);

      if (!thread_default_context)
        g_main_context_pop_thread_default (context);

      g_main_context_release (context);

      /* Unblock a potentially waiting
       * grustna_main_loop_run_thread_local() */
      g_cond_broadcast (&rust_context_released_cond);

      return TRUE;
    }
  else
    {
      /* Shunt the call to the loop thread
       * and wait for it to complete. */

      RustCallData *call_data;
      RustCallStatus status;
      GSource *idle;

      call_data = g_slice_new0 (RustCallData);
      call_data->func = func;
      call_data->param = data;
      call_data->context = g_main_context_ref (context);
      call_data->ref_count = 3;
      call_data->minder_backoff = 1 * G_TIME_SPAN_MILLISECOND;
      call_data->status = RUST_CALL_PENDING;

      idle = g_idle_source_new ();
      g_source_set_priority (idle, G_PRIORITY_DEFAULT);
      g_source_set_callback (idle, loop_callback, call_data, NULL);
      g_source_attach (idle, context);
      call_data->source = idle;

      g_cond_init (&call_data->return_cond);

      add_call_minder (call_data);

      g_mutex_lock (&call_mutex);
      while ((status = call_data->status) == RUST_CALL_PENDING)
        g_cond_wait (&call_data->return_cond, &call_mutex);
      g_mutex_unlock (&call_mutex);

      call_data_unref (call_data);

      return status == RUST_CALL_RETURNED;
    }
}

GMainLoop *
grustna_main_loop_new_thread_local ()
{
  return g_main_loop_new (get_rust_thread_context (), FALSE);
}

void
grustna_main_loop_run_thread_local (GMainLoop *loop)
{
  GMainContext *context;
  gboolean context_acquired;

  context = g_main_loop_get_context (loop);

  context_acquired = g_main_context_acquire (context);
  while (!context_acquired)
    context_acquired = g_main_context_wait (context,
                                            &rust_context_released_cond,
                                            &rust_context_mutex);

  g_main_context_push_thread_default (context);
  g_main_loop_run (loop);
  g_main_context_pop_thread_default (context);

  g_main_context_release (context);
}
