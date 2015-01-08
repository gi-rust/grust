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

use grust::gtype::GType;
use grust::marker;
use grust::object;
use grust::wrap;

#[repr(C)]
pub struct TypeInstance {
    raw: raw::GTypeInstance,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for TypeInstance {
    type Raw = raw::GTypeInstance;
}

#[repr(C)]
pub struct Object {
    raw: raw::GObject,
    _marker: marker::ObjectMarker
}

unsafe impl wrap::Wrapper for Object {
    type Raw = raw::GObject;
}

pub mod cast {
    use grust::object;

    pub trait AsObject {
        fn as_gobject_object(&self) -> &super::Object;
        fn as_mut_gobject_object(&mut self) -> &mut super::Object;
    }

    impl<T> AsObject for T where T: object::Upcast<super::Object> {

        #[inline]
        fn as_gobject_object(&self) -> &super::Object { self.upcast() }

        #[inline]
        fn as_mut_gobject_object(&mut self) -> &mut super::Object { self.upcast_mut() }
    }
}

#[allow(missing_copy_implementations)]
pub mod raw {
    use grust::gtype::raw::GType;
    use grust::types::{gpointer, guint};

    #[repr(C)]
    pub struct GTypeInstance {
        g_class: gpointer
    }

    #[repr(C)]
    pub struct GObject {
        g_type_instance: GTypeInstance,
        ref_count: guint,
        data: gpointer
    }

    #[link_name="gobject-2.0"]
    extern {
        pub fn g_object_get_type() -> GType;
    }
}

unsafe impl object::ObjectType for Object {
    fn get_type(_: Option<&Self>) -> GType {
        unsafe {
            GType::new(raw::g_object_get_type())
        }
    }
}
