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

use gobject as ffi;

use std::boxed::into_raw as box_into_raw;
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

extern "C" fn box_copy<T>(raw: gpointer) -> gpointer
    where T: Clone
{
    let boxed: Box<T> = unsafe { Box::from_raw(raw as *mut T) };
    let copy: Box<T> = boxed.clone();
    unsafe {
        // Prevent the original value from being dropped
        box_into_raw(boxed);
        box_into_raw(copy) as gpointer
    }
}

extern "C" fn box_free<T>(raw: gpointer) {
    let boxed: Box<T> = unsafe { Box::from_raw(raw as *mut T) };
    mem::drop(boxed);
}

pub fn register_box_type<T>(name: &str) -> GType where T: Clone + Send {
    let c_name = CString::new(name).unwrap();
    let raw = unsafe {
        ffi::g_boxed_type_register_static(c_name.as_ptr(),
                                          box_copy::<T>,
                                          box_free::<T>)
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
        Box::from_raw(raw as *mut T)
    }

    unsafe fn into_ptr(self) -> gpointer {
        box_into_raw(self) as gpointer
    }
}
