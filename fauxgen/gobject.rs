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

#![crate_name = "grust-GObject-2_0"]

#![crate_type = "lib"]

extern crate grust;
extern crate "grust-GLib-2_0" as glib;

use grust::gtype;
use grust::marker;
use grust::object;
use grust::types::{gpointer,guint};

#[repr(C)]
pub struct TypeInstance {
    g_class: gpointer,

    _marker: marker::ObjectMarker
}

#[repr(C)]
pub struct Object {
    g_type_instance: TypeInstance,
    ref_count      : guint,
    data           : gpointer
}

pub mod cast {
    pub trait Object {
        fn as_gobject_object<'a>(&'a self) -> &'a super::Object;
        fn as_mut_gobject_object<'a>(&'a mut self) -> &'a mut super::Object;
    }
}

pub mod raw {
    use grust::gtype::GType;

    pub type GTypeInstance = super::TypeInstance;
    pub type GObject = super::Object;

    #[link_name="gobject-2.0"]
    extern {
        pub fn g_object_get_type() -> GType;
    }
}

impl object::ObjectType for Object {
    fn get_type(&self) -> gtype::GType {
        unsafe {
            raw::g_object_get_type()
        }
    }
}

impl cast::Object for Object {
    fn as_gobject_object<'a>(&'a self) -> &'a Object { self }
    fn as_mut_gobject_object<'a>(&'a mut self) -> &'a mut Object { self }
}
