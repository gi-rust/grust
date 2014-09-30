/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

#![crate_name = "grust"]

#![crate_type = "lib"]

#![desc = "GObject introspection bindings"]
#![license = "LGPL 2.1"]

#![feature(unsafe_destructor)]

extern crate libc;

pub mod error;
pub mod gstr;
pub mod gtype;
pub mod mainloop;
pub mod native;
pub mod object;
pub mod quark;
pub mod refcount;
pub mod types;
pub mod util;

mod ffi;
mod plumbing;
