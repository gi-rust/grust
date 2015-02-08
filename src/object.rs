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

use gobject as ffi;
use gtype;
use gtype::GType;
use refcount::Refcount;
use types::gpointer;
use wrap::Wrapper;

use std::mem::transmute;

pub unsafe trait ObjectType {
    fn get_type() -> GType;
}

impl<T> Refcount for T where T: ObjectType + Wrapper {

    unsafe fn inc_ref(&self) {
        ffi::g_object_ref(self.as_mut_ptr() as gpointer);
    }

    unsafe fn dec_ref(&self) {
        ffi::g_object_unref(self.as_mut_ptr() as gpointer);
    }
}

pub trait Upcast<T> {
    fn upcast(&self) -> &T;
}

impl<T> Upcast<T> for T {

    #[inline]
    fn upcast(&self) -> &T { self }
}

pub fn type_of<T>() -> GType where T: ObjectType {
    <T as ObjectType>::get_type()
}

pub fn is_instance_of<T, U>(object: &T) -> bool
    where T: ObjectType, U: ObjectType
{
    gtype::check_instance_is_a(object, type_of::<U>())
}

fn assert_instance_of<T, U>(object: &T)
    where T: ObjectType, U: ObjectType
{
    let dest_type = type_of::<U>();
    assert!(gtype::check_instance_is_a(object, dest_type),
            "invalid cast to type {}", dest_type.name())
}

pub fn cast<T, U>(source: &T) -> &U
    where T: ObjectType, U: ObjectType
{
    assert_instance_of::<T, U>(source);
    unsafe { transmute(source) }
}
