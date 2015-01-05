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

use gtype;
use gtype::GType;
use ffi;
use refcount::{Refcount, RefcountFuncs};

use std::mem::transmute;

pub unsafe trait ObjectType {
    fn get_type(_fixme_ufcs: Option<&Self>) -> GType;
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

pub fn type_of<T>() -> GType
    where T: ObjectType
{
    let fixme_ufcs: Option<&T> = None;
    ObjectType::get_type(fixme_ufcs)
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
            "invalid cast to type {}", dest_type)
}

pub fn cast<T, U>(source: &T) -> &U
    where T: ObjectType, U: ObjectType
{
    assert_instance_of::<T, U>(source);
    unsafe { transmute(source) }
}

pub fn cast_mut<T, U>(source: &mut T) -> &mut U
    where T: ObjectType, U: ObjectType
{
    assert_instance_of::<T, U>(source);
    unsafe { transmute(source) }
}
