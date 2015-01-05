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
#![allow(improper_ctypes)]

use gtype::GType;
use error;
use mainloop;
use types::{gboolean, gchar, gdouble, gfloat, gint, glong, gssize};
use types::{guchar, guint, gulong};
use types::gpointer;
use value;

pub type GError = error::raw::GError;
pub type GMainContext = mainloop::MainContext;
pub type GMainLoop = mainloop::MainLoop;
pub type GQuark = u32;
pub type GValue = value::Value;

#[repr(C)]
pub struct GTypeInstance;

#[link(name = "glib-2.0")]
extern {
    pub fn g_free(mem: gpointer);
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
    pub fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    pub fn g_quark_to_string(quark: GQuark) -> *const gchar;
    pub fn g_strdup(str: *const gchar) -> *mut gchar;
    pub fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
}

#[link(name = "gobject-2.0")]
extern {
    pub fn g_object_ref(obj: gpointer) -> gpointer;
    pub fn g_object_unref(obj: gpointer);
    pub fn g_type_check_instance_is_a(instance  : *const GTypeInstance,
                                  iface_type: GType) -> gboolean;
    pub fn g_type_name(t: GType) -> *const gchar;
    pub fn g_value_copy(src: *const GValue, dst: *mut GValue);
    pub fn g_value_get_boolean(value: *const GValue) -> gboolean;
    pub fn g_value_get_char(value: *const GValue) -> gchar;
    pub fn g_value_get_double(value: *const GValue) -> gdouble;
    pub fn g_value_get_float(value: *const GValue) -> gfloat;
    pub fn g_value_get_int(value: *const GValue) -> gint;
    pub fn g_value_get_int64(value: *const GValue) -> i64;
    pub fn g_value_get_long(value: *const GValue) -> glong;
    pub fn g_value_get_object(value: *const GValue) -> gpointer;
    pub fn g_value_get_schar(value: *const GValue) -> i8;
    pub fn g_value_get_string(value: *const GValue) -> *const gchar;
    pub fn g_value_get_uchar(value: *const GValue) -> guchar;
    pub fn g_value_get_uint(value: *const GValue) -> guint;
    pub fn g_value_get_uint64(value: *const GValue) -> u64;
    pub fn g_value_get_ulong(value: *const GValue) -> gulong;
    pub fn g_value_init(value: *mut GValue, type_id: GType) -> *mut GValue;
    pub fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    pub fn g_value_set_char(value: *mut GValue, v_char: gchar);
    pub fn g_value_set_double(value: *mut GValue, v_double: gdouble);
    pub fn g_value_set_float(value: *mut GValue, v_float: gfloat);
    pub fn g_value_set_int(value: *mut GValue, v_int: gint);
    pub fn g_value_set_int64(value: *mut GValue, v_int: i64);
    pub fn g_value_set_long(value: *mut GValue, v_long: glong);
    pub fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    pub fn g_value_set_schar(value: *mut GValue, v_char: i8);
    pub fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    pub fn g_value_set_static_string(value: *mut GValue, v_string: *const gchar);
    pub fn g_value_set_uchar(value: *mut GValue, v_char: guchar);
    pub fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    pub fn g_value_set_uint64(value: *mut GValue, v_uint: u64);
    pub fn g_value_set_ulong(value: *mut GValue, v_ulong: gulong);
    pub fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
    pub fn g_value_unset(value: *mut GValue);
}
