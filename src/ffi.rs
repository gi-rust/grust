// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

// https://github.com/rust-lang/rust/issues/17679
#![allow(ctypes)]

use types::{gboolean,gchar,gpointer};
use gtype::GType;

use error;
use mainloop;
use native;
use quark;

pub type GError = error::raw::GError;

pub type GMainContext = mainloop::MainContext;

pub type GMainLoop = native::MainLoop;

pub type GQuark = quark::Quark;

#[repr(C)]
pub struct GTypeInstance;

#[link(name = "glib-2.0")]
extern {
    pub fn g_free(mem: gpointer);
    pub fn g_strdup(str: *const gchar) -> *mut gchar;
    pub fn g_error_copy(error: *const GError) -> *mut GError;
    pub fn g_error_free(error: *mut GError);
    pub fn g_main_context_new() -> *mut GMainContext;
    pub fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    pub fn g_main_context_unref(context: *mut GMainContext);
    pub fn g_main_context_default() -> *mut GMainContext;
    pub fn g_main_context_push_thread_default(context: *mut GMainContext);
    pub fn g_main_context_pop_thread_default(context: *mut GMainContext);
    pub fn g_main_loop_new(ctx: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    pub fn g_main_loop_ref(l: *mut GMainLoop) -> *mut GMainLoop;
    pub fn g_main_loop_unref(l: *mut GMainLoop);
    pub fn g_main_loop_get_context(l: *mut GMainLoop) -> *mut GMainContext; 
    pub fn g_main_loop_run(l: *mut GMainLoop);
    pub fn g_main_loop_quit(l: *mut GMainLoop);
}

#[link(name = "gobject-2.0")]
extern {
    pub fn g_object_ref(obj: gpointer) -> gpointer;
    pub fn g_object_unref(obj: gpointer);
    pub fn g_type_check_instance_is_a(instance  : *const GTypeInstance,
                                  iface_type: GType) -> gboolean;
    pub fn g_type_name(t: GType) -> *const gchar;
}
