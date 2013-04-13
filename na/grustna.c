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

#define G_LOG_DOMAIN "Grust"

#include <glib.h>

typedef void (*RustFunc) (gpointer param, GMainContext *context);

typedef struct _RustCallData RustCallData;
struct _RustCallData
{
  RustFunc func;
  gpointer param;
  GCond return_cond;
  volatile gboolean returned;
};

/* This could be a per-call mutex, but the callers are going to wait on
 * the likely single main loop thread anyway.
 * The balance of reduced contention vs. extra init/cleanup calls
 * would need to be profiled. */
static GMutex call_mutex;

static gboolean loop_callback (gpointer data)
{
  RustCallData *call_data = data;
  GMainContext *context;

  context = g_main_context_get_thread_default ();
  if (context == NULL)
    context = g_main_context_default ();

  call_data->func (call_data->param, context);

  g_mutex_lock (&call_mutex);
  call_data->returned = TRUE;
  g_cond_signal (&call_data->return_cond);
  g_mutex_unlock (&call_mutex);

  return FALSE;
}

void
grustna_call (RustFunc func, gpointer data, GMainContext *context)
{
  GMainContext *thread_context;

  g_return_if_fail (func != NULL);

  /* This code is largely copied from g_main_context_invoke_full() */

  if (context == NULL)
    context = g_main_context_default ();

  if (g_main_context_is_owner (context))
    {
      /* Fastest path: the caller is in the same thread where some code
       * is supposedly driving the loop context affine to this call. */
      func (data, context);
      return;
    }

  thread_context = g_main_context_get_thread_default ();
  if (thread_context == NULL)
    thread_context = g_main_context_default ();

  if (context == thread_context && g_main_context_acquire (context))
    {
      /* Here, we get to exclusively use the thread's default context
       * that is not (yet) driven by an event loop.
       * This is perfectly OK for non-async functions on objects affine
       * to this context, and matches the behavior of GIO-style async calls
       * that rely on the thread-default context to be eventually driven
       * in order to complete. */
      func (data, context);
      g_main_context_release (context);
    }
  else
    {
      /* We are out of fast options. Shunt the call to the loop thread
       * and wait for it to complete. */

      RustCallData call_data;
      GSource *idle;

      call_data.func = func;
      call_data.param = data;

      g_cond_init (&call_data.return_cond);

      idle = g_idle_source_new ();
      g_source_set_priority (idle, G_PRIORITY_DEFAULT);
      g_source_set_callback (idle, loop_callback, &call_data, NULL);
      g_source_attach (idle, context);
      g_source_unref (idle);

      g_mutex_lock (&call_mutex);
      while (!call_data.returned)
        g_cond_wait (&call_data.return_cond, &call_mutex);
      g_mutex_unlock (&call_mutex);

      g_cond_clear (&call_data.return_cond);
    }
}
