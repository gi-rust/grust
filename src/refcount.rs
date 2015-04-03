// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use wrap::Wrapper;

use std::mem;
use std::ops::Deref;

pub trait Refcount {
    unsafe fn inc_ref(&self);
    unsafe fn dec_ref(&self);
}

pub struct Ref<T> where T: Refcount {
    ptr: *const T
}

unsafe impl<T> Send for Ref<T> where T: Refcount + Send { }
unsafe impl<T> Sync for Ref<T> where T: Refcount + Sync { }

impl<T> Ref<T> where T: Refcount {

    pub fn new(source: &T) -> Ref<T> {
        unsafe {
            source.inc_ref();
        }
        Ref { ptr: source }
    }
}

impl<T> Ref<T> where T: Refcount + Wrapper {
    pub unsafe fn from_raw(ptr: *mut <T as Wrapper>::Raw) -> Ref<T> {
        Ref { ptr: ptr as *const T }
    }
}

pub unsafe fn ref_into_raw<T>(r: Ref<T>) -> *mut <T as Wrapper>::Raw
    where T: Refcount + Wrapper
{
    let ptr = r.ptr;
    mem::forget(r);
    ptr as *mut _
}

impl<T> Drop for Ref<T> where T: Refcount {
    fn drop(&mut self) {
        unsafe {
            self.dec_ref();
        }
    }
}

impl<T> Clone for Ref<T> where T: Refcount {

    fn clone(&self) -> Ref<T> {
        Ref::new(self.deref())
    }

    fn clone_from(&mut self, source: &Ref<T>) {
        unsafe {
            source.inc_ref();
            self.dec_ref();
        }
        self.ptr = source.ptr;
    }
}

impl<T> Deref for Ref<T> where T: Refcount {

    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
