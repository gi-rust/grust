/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use std::kinds::marker;
use types::gpointer;

pub type RefFunc = unsafe extern "C" fn(gpointer) -> gpointer;
pub type UnrefFunc = unsafe extern "C" fn(gpointer);
pub type RefcountFuncs = (*const RefFunc, *const UnrefFunc);

pub trait Refcount {
    fn refcount_funcs(&self) -> &'static RefcountFuncs;

    unsafe fn inc_ref(&mut self) {
        let (ref_func, _) = *self.refcount_funcs();
        let p: *mut Self = self;
        (*ref_func)(p as gpointer);
    }

    unsafe fn dec_ref(&mut self) {
        let (_, unref_func) = *self.refcount_funcs();
        let p: *mut Self = self;
        (*unref_func)(p as gpointer);
    }
}

pub struct Ref<T: Refcount> {
    ptr: *mut T,
    no_send: marker::NoSend,
    no_sync: marker::NoSync
}

pub mod raw {

    use super::{Ref,Refcount};
    use std::kinds::marker;

    pub unsafe fn ref_from_ptr<T: Refcount>(p: *mut T) -> Ref<T> {
        Ref {
            ptr: p,
            no_send: marker::NoSend,
            no_sync: marker::NoSync
        }
    }

}

pub struct Unowned<'a, T: 'a + Refcount> {
    ptr: *mut T,
    lifetime_marker: marker::ContravariantLifetime<'a>
}

unsafe fn make_ref<T: Refcount>(p: *mut T) -> Ref<T> {
    (*p).inc_ref();
    raw::ref_from_ptr(p)
}

unsafe fn make_unowned<'a, T: 'a + Refcount>(p: *mut T) -> Unowned<'a, T> {
    Unowned {
        ptr: p,
        lifetime_marker: marker::ContravariantLifetime
    }
}

impl<T: Refcount> Ref<T> {
    pub fn new(source: &Unowned<T>) -> Ref<T> {
        unsafe { make_ref(source.ptr) }
    }
    pub fn borrow<'a>(&'a self) -> &'a T { unsafe { &*self.ptr } }
    pub fn borrow_mut<'a>(&'a mut self) -> &'a mut T { unsafe { &mut *self.ptr } }
}

impl<'a, T: 'a + Refcount> Unowned<'a, T> {
    pub fn new_ref(&self) -> Ref<T> { unsafe { make_ref(self.ptr) } }
}

#[unsafe_destructor]
impl<T: Refcount> Drop for Ref<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).dec_ref();
        }
    }
}

impl<T: Refcount> Clone for Ref<T> {

    fn clone(&self) -> Ref<T> { unsafe { make_ref(self.ptr) } }

    fn clone_from(&mut self, source: &Ref<T>) {
        unsafe {
            (*source.ptr).inc_ref();
            (*self.ptr).dec_ref();
            self.ptr = source.ptr;
        }
    }
}

impl<T: Refcount> Deref<T> for Ref<T> {
    fn deref<'a>(&'a self) -> &'a T { unsafe { &*self.ptr } }
}

impl<T: Refcount> DerefMut<T> for Ref<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T { unsafe { &mut *self.ptr } }
}

impl<'b, T: Refcount> Deref<T> for Unowned<'b, T> {
    fn deref<'a>(&'a self) -> &'a T { unsafe { &*self.ptr } }
}

impl<'b, T: Refcount> DerefMut<T> for Unowned<'b, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T { unsafe { &mut *self.ptr } }
}
