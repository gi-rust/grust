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
use ll::*;
use plumbing;
use plumbing::{GObject,GMainContext};
use types::*;

pub use types::{GType};

trait ObjectType {
    fn get_type() -> GType;
}

pub struct Interface<T> {
    priv bare: plumbing::Object
}

pub struct Reference<T> {
    priv iface: Interface<T>
}

impl<T> Interface<T> {
    pub fn new_ref(&self) -> Reference<T> {
        unsafe {
            self.bare.inc_ref();
        }
        Reference { iface: Interface { bare: self.bare } }
    }

    pub fn cast<'r, U: ObjectType>(&'r self) -> &'r Interface<U> { cast(self) }

    pub unsafe fn raw(&self) -> *T { self.bare.raw() as *T }
    pub unsafe fn context(&self) -> *GMainContext { self.bare.context() }
}

impl<T> Reference<T> {
    pub fn interface<'r>(&'r self) -> &'r Interface<T> { &self.iface }
    pub fn as_interface<U>(&self, f: &fn(&Interface<T>) -> U) -> U {
        f(&self.iface)
    }
}

#[unsafe_destructor]
impl<T> Drop for Interface<T> {
    /* Non-copyable */
    fn finalize(&self) { }
}

#[unsafe_destructor]
impl<T> Drop for Reference<T> {
    fn finalize(&self) {
        unsafe {
            self.iface.bare.dec_ref();
        }
    }
}

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Reference<T> {
        self.iface.new_ref()
    }
}

pub fn cast<'r, T, U: ObjectType>(t: &'r Interface<T>) -> &'r Interface<U> {
    unsafe {
        let inst = t.bare.type_instance();
        let dest_type = ObjectType::get_type::<U>();
        if !(g_type_check_instance_is_a(inst, dest_type) as bool) {
            fail!(fmt!("invalid cast to type `%s'",
                       str::raw::from_c_str(g_type_name(dest_type))));
        }
        cast::transmute(t)
    }
}

pub unsafe fn make_interface<T>(obj: *T, ctx: *GMainContext)
                               -> Interface<T> {
    Interface {
        bare: plumbing::get_object(obj as *GObject, ctx)
    }
}

pub unsafe fn take_object<T>(obj: *T, ctx: *GMainContext)
                            -> Reference<T> {
    Reference {
        iface: Interface {
            bare: plumbing::take_object(obj as *GObject, ctx)
        }
    }
}
