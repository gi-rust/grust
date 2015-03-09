// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

#![crate_name = "grust"]
#![crate_type = "lib"]

#![allow(unstable_features)]

#![feature(alloc)]
#![feature(core)]
#![feature(libc)]
#![feature(unsafe_destructor)]
#![feature(unsafe_no_drop_flag)]

extern crate libc;
extern crate "glib-2_0-sys" as glib;
extern crate "gobject-2_0-sys" as gobject;

#[macro_use]
mod macros;

pub mod boxed;
pub mod enumeration;
pub mod error;
pub mod flags;
pub mod gstr;
pub mod gtype;
pub mod mainloop;
pub mod marker;
pub mod object;
pub mod quark;
pub mod refcount;
pub mod types;
pub mod util;
pub mod value;
pub mod wrap;
