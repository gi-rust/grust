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

use gstr;
use gtype::GType;
use ffi;
use ffi::GTypeInstance;
use refcount::{Refcount, RefcountFuncs};
use util::is_true;

use std::mem::transmute;

pub trait ObjectType {
    fn get_type(&self) -> GType;
}

const REFCOUNT_FUNCS: &'static RefcountFuncs = &(
        &ffi::g_object_ref,
        &ffi::g_object_unref
    );

impl<T> Refcount for T where T: ObjectType {
    fn refcount_funcs(&self) -> &'static RefcountFuncs {
        return REFCOUNT_FUNCS;
    }
}

pub trait Upcast<T> {
    fn upcast(&self) -> &T;
    fn upcast_mut(&mut self) -> &mut T;
}

impl<T> Upcast<T> for T {

    #[inline]
    fn upcast(&self) -> &T { self }

    #[inline]
    fn upcast_mut(&mut self) -> &mut T { self }
}

pub fn cast<'a, T: ObjectType, U: ObjectType>(source: &'a T)
                                             -> &'a U {
    unsafe {
        let inst = source as *const T as *const GTypeInstance;
        let dest: &'a U = transmute(source);
        let dest_type = dest.get_type();
        assert!(is_true(ffi::g_type_check_instance_is_a(inst, dest_type)),
                "invalid cast to type {}",
                gstr::parse_as_utf8(&ffi::g_type_name(dest_type)).unwrap());
        dest
    }
}

pub fn cast_mut<'a, T: ObjectType, U: ObjectType>(source: &'a mut T)
                                                  -> &'a mut U {
    unsafe {
        let inst = source as *mut T as *const GTypeInstance;
        let dest: &'a mut U = transmute(source);
        let dest_type = dest.get_type();
        assert!(is_true(ffi::g_type_check_instance_is_a(inst, dest_type)),
                "invalid cast to type {}",
                gstr::parse_as_utf8(&ffi::g_type_name(dest_type)).unwrap());
        dest
    }
}
