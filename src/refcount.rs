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

use types::gpointer;
use wrap::Wrapper;

use std::mem;
use std::ops::{Deref, DerefMut};

pub type RefFunc = unsafe extern "C" fn(gpointer) -> gpointer;
pub type UnrefFunc = unsafe extern "C" fn(gpointer);
pub type RefcountFuncs = (*const RefFunc, *const UnrefFunc);

pub trait Refcount {
    fn refcount_funcs(&self) -> &'static RefcountFuncs;
}

pub struct Ref<T> where T: Refcount {
    plumbing: RefImpl
}

pub struct SyncRef<T> where T: Refcount {
    plumbing: RefImpl
}

unsafe impl<T> Send for SyncRef<T> where T: Refcount + Send + Sync { }
unsafe impl<T> Sync for SyncRef<T> where T: Refcount + Send + Sync { }

impl<T> Ref<T> where T: Refcount {

    pub fn new(source: &T) -> Ref<T> {
        let p = source as *const T as gpointer;
        let funcs = source.refcount_funcs();
        Ref {
            plumbing: unsafe { RefImpl::new(p, funcs) }
        }
    }
}

impl<T> Ref<T> where T: Refcount + Wrapper {

    pub unsafe fn from_raw(ptr: *mut <T as Wrapper>::Raw) -> Ref<T> {
        Ref {
            plumbing: RefImpl { ptr: ptr as gpointer }
        }
    }
}

impl<T> SyncRef<T> where T: Refcount + Send + Sync {

    pub fn new(source: &T) -> SyncRef<T> {
        let p = source as *const T as gpointer;
        let funcs = source.refcount_funcs();
        SyncRef {
            plumbing: unsafe { RefImpl::new(p, funcs) }
        }
    }
}

impl<T> SyncRef<T> where T: Refcount + Wrapper + Send + Sync {

    pub unsafe fn from_raw(ptr: *mut <T as Wrapper>::Raw) -> SyncRef<T> {
        SyncRef {
            plumbing: RefImpl { ptr: ptr as gpointer }
        }
    }
}

struct RefImpl {
    ptr: gpointer
}

impl RefImpl {

    unsafe fn new(p: gpointer, funcs: &RefcountFuncs) -> RefImpl {
        let (inc_ref, _) = *funcs;
        (*inc_ref)(p);
        RefImpl { ptr: p }
    }

    unsafe fn impl_drop(&mut self, funcs: &RefcountFuncs) {
        let (_, dec_ref) = *funcs;
        (*dec_ref)(self.ptr);
    }

    unsafe fn impl_clone_from(&mut self,
                              source: &RefImpl,
                              funcs: &RefcountFuncs)
    {
        let (inc_ref, dec_ref) = *funcs;
        (*inc_ref)(source.ptr);
        (*dec_ref)(self.ptr);
        self.ptr = source.ptr;
    }
}

#[unsafe_destructor]
impl<T> Drop for Ref<T> where T: Refcount {
    fn drop(&mut self) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_drop(funcs);
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for SyncRef<T> where T: Refcount {
    fn drop(&mut self) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_drop(funcs);
        }
    }
}

impl<T> Clone for Ref<T> where T: Refcount {

    fn clone(&self) -> Ref<T> {
        Ref::new(self.deref())
    }

    fn clone_from(&mut self, source: &Ref<T>) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_clone_from(&source.plumbing, funcs);
        }
    }
}

impl<T> Clone for SyncRef<T> where T: Refcount + Send + Sync {

    fn clone(&self) -> SyncRef<T> {
        SyncRef::new(self.deref())
    }

    fn clone_from(&mut self, source: &SyncRef<T>) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_clone_from(&source.plumbing, funcs);
        }
    }
}

impl<T> Deref for Ref<T> {

    type Target = T;

    fn deref(&self) -> &T {
        let p = self.plumbing.ptr as *const T;
        unsafe { mem::copy_lifetime(self, &*p) }
    }
}

impl<T> Deref for SyncRef<T> {

    type Target = T;

    fn deref(&self) -> &T {
        let p = self.plumbing.ptr as *const T;
        unsafe { mem::copy_lifetime(self, &*p) }
    }
}

impl<T> DerefMut for Ref<T> {

    fn deref_mut(&mut self) -> &mut T {
        let p = self.plumbing.ptr as *mut T;
        unsafe { mem::copy_mut_lifetime(self, &mut *p) }
    }
}

impl<T> DerefMut for SyncRef<T> {

    fn deref_mut(&mut self) -> &mut T {
        let p = self.plumbing.ptr as *mut T;
        unsafe { mem::copy_mut_lifetime(self, &mut *p) }
    }
}
