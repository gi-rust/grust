/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

typedef gpointer (*RustFunc) (gpointer user_data);

typedef struct _RustCallData RustCallData;
struct _RustCallData
{
  RustFunc func;
  gpointer param;
  gpointer retval;
  GCond return_cond;
  volatile gboolean returned;
};

static gpointer
grustna_run (gpointer thread_data)
{
  GMainContext *context = thread_data;
  GMainLoop *event_loop;
  gboolean warned_once = FALSE;

  g_main_context_push_thread_default (context);

  for (;;)
    {
      event_loop = g_main_loop_new (context, FALSE);
      g_main_loop_run (event_loop);
      if (!warned_once)
        {
          g_warning ("The grust event loop quit somehow, will be recreated. "
                     "This warning will not be repeated.");
          warned_once = TRUE;
        }
      g_main_loop_unref (event_loop);
    }

  return NULL;
}

static gpointer
init_context ()
{
  GMainContext *context;
  GThread *thread;

  context = g_main_context_new ();

  thread = g_thread_new ("grustna", grustna_run, context);

  g_thread_unref (thread);

  return context;
}

static GMainContext *
get_event_thread_context ()
{
  static GOnce init_once = G_ONCE_INIT;

  g_once (&init_once, init_context, NULL);

  return init_once.retval;
}

/* Could be a per-call mutex, but the callers are going to wait on
 * the global event thread anyway */
static GMutex call_mutex;

static gboolean loop_callback (gpointer data)
{
  RustCallData *call_data = data;

  call_data->retval = call_data->func (call_data->param);

  g_mutex_lock (&call_mutex);
  call_data->returned = TRUE;
  g_cond_signal (&call_data->return_cond);
  g_mutex_unlock (&call_mutex);

  return FALSE;
}

gpointer
grustna_call (RustFunc func, gpointer param)
{
  RustCallData call_data;
  GSource *idle;

  call_data.func = func;
  call_data.param = param;
  call_data.retval = NULL;

  g_cond_init (&call_data.return_cond);

  idle = g_idle_source_new ();
  g_source_set_priority (idle, G_PRIORITY_DEFAULT);
  g_source_set_callback (idle, loop_callback, &call_data, NULL);
  g_source_attach (idle, get_event_thread_context ());
  g_source_unref (idle);

  g_mutex_lock (&call_mutex);
  while (!call_data.returned)
    g_cond_wait (&call_data.return_cond, &call_mutex);
  g_mutex_unlock (&call_mutex);

  g_cond_clear (&call_data.return_cond);

  return call_data.retval;
}
