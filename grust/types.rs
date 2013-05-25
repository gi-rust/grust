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

pub type gboolean       = libc::c_int;
pub type gchar          = libc::c_char;
pub type gint           = libc::c_int;
pub type guint          = libc::c_uint;
pub type gsize          = libc::size_t;

pub static FALSE: gboolean = 0;
pub static TRUE:  gboolean = !FALSE;

pub struct GMainContext;
pub struct GMainLoop;
pub struct GObject;
pub struct GTypeInstance;
pub type GType = gsize;