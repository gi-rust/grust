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

// This module provides types that are intrinsic in GIR, so they do not
// get defined through other types. It should ultimately have a name defined
// for every basic type listed in the documentation:
// https://live.gnome.org/GObjectIntrospection/Annotations#Default_Basic_Types
//
// Exceptions are:
// 1. Fixed-size integer types. These have straightforward machine-independent
//    counterparts in Rust.
// 2. Strings annotated as "utf8" or "filename". These types are not named
//    as such in the C API. There is no limitation for an introspected API
//    against providing its own "utf8" or "filename", so these types are
//    disambiguated in the generated code by qualifying their names with
//    a separate module, gstr.
//
// Rust aliases are needed for machine-dependent basic types used in GIR,
// since the GLib types are not necessarily identical to their Rust namesakes
// (the issue similarly addressed by std::libc::c_int and the like).
// Usage of the GLib typedef names prevents potential name conflicts,
// because the introspected C API is likely to compile with the GLib type
// definitions, and it's highly unlikely for these names to be given to
// something else via GI annotations.
//
// Any other GLib-derived types used by the bindings require some name
// disambiguation with definitions for the same C types that are emitted
// in "raw" namespaces of the generated GLib API crates (which are dependent,
// as any others, on the grust crate, so we cannot refer to their types here
// without creating a circular build dependency). Those type names should
// not be defined so as to be pulled in by 'use grust::types::*'.

pub type gboolean       = libc::c_int;
pub type gchar          = libc::c_char;
pub type gint           = libc::c_int;
pub type guint          = libc::c_uint;
pub type gsize          = libc::size_t;

pub static FALSE: gboolean = 0;
pub static TRUE:  gboolean = !FALSE;
