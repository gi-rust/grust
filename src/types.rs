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


// This module provides types that are intrinsic in GIR, so they do not
// get defined through other types. It should ultimately have a name defined
// for every basic type listed in the documentation:
// https://wiki.gnome.org/Projects/GObjectIntrospection/Annotations#Default_Basic_Types
//
// Exceptions are:
// 1. Fixed-size integer types. These have straightforward machine-independent
//    counterparts in Rust.
// 2. Strings annotated as "utf8" or "filename". These types are not named
//    as such in the C API, being only aliases for gchar*.
//    Their representation in Rust is quite intricate and involves multiple
//    types, so they each get their own module.
//
// Rust aliases are needed for machine-dependent basic types used in GIR,
// since the GLib types are not necessarily identical to their Rust namesakes
// (the issue similarly addressed by libc::c_int and the like).
// GIR uses the GLib names for these types as well.

#![allow(non_camel_case_types)]

use libc;

pub type gboolean       = libc::c_int;
pub type gchar          = libc::c_char;
pub type guchar         = libc::c_uchar;
pub type gint           = libc::c_int;
pub type guint          = libc::c_uint;
pub type glong          = libc::c_long;
pub type gulong         = libc::c_ulong;
pub type gsize          = libc::size_t;
pub type gssize         = libc::ssize_t;
pub type gfloat         = libc::c_float;
pub type gdouble        = libc::c_double;
pub type gpointer       = *mut   libc::c_void;
pub type gconstpointer  = *const libc::c_void;

pub const FALSE: gboolean = 0;
pub const TRUE : gboolean = 1;
