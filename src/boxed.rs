// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use gtype::GType;
use types::gpointer;
use util::{box_free, box_from_pointer, box_into_pointer};

use gobject as ffi;

use std::ffi::CString;
use std::mem;

pub trait BoxedType {
    fn get_type() -> GType;
    unsafe fn from_ptr(ptr: gpointer) -> Self;
    unsafe fn into_ptr(self) -> gpointer;
}

pub fn type_of<T>() -> GType where T: BoxedType
{
    <T as BoxedType>::get_type()
}

unsafe extern "C" fn box_copy<T>(raw: gpointer) -> gpointer
    where T: Clone
{
    let boxed: Box<T> = box_from_pointer(raw);
    let copy: Box<T> = boxed.clone();
    // Prevent the original value from being dropped
    mem::forget(boxed);
    box_into_pointer(copy)
}

unsafe fn into_boxed_copy_func(callback: unsafe extern "C" fn(gpointer) -> gpointer)
                              -> ffi::GBoxedCopyFunc
{
    mem::transmute(callback)
}

unsafe fn into_boxed_free_func(callback: unsafe extern "C" fn(gpointer))
                              -> ffi::GBoxedFreeFunc
{
    mem::transmute(callback)
}

pub fn register_box_type<T>(name: &str) -> GType
    where T: Clone + Send + 'static
{
    let c_name = CString::new(name).unwrap();
    let raw = unsafe {
        ffi::g_boxed_type_register_static(c_name.as_ptr(),
                into_boxed_copy_func(box_copy::<T>),
                into_boxed_free_func(box_free::<T>))
    };
    assert!(raw != 0, "failed to register type \"{}\"", name);
    unsafe { GType::from_raw(raw) }
}

pub unsafe trait BoxRegistered : Clone + Send {
    fn box_type() -> GType;
}

impl<T> BoxedType for Box<T> where T: BoxRegistered {

    fn get_type() -> GType {
        <T as BoxRegistered>::box_type()
    }

    unsafe fn from_ptr(raw: gpointer) -> Box<T> {
        box_from_pointer(raw)
    }

    unsafe fn into_ptr(self) -> gpointer {
        box_into_pointer(self)
    }
}
