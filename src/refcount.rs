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

use types::{gpointer,gconstpointer};

use std::kinds::marker;
use std::ops::{Deref, DerefMut};

pub type RefFunc = unsafe extern "C" fn(gpointer) -> gpointer;
pub type UnrefFunc = unsafe extern "C" fn(gpointer);
pub type RefcountFuncs = (*const RefFunc, *const UnrefFunc);

pub trait Refcount {
    fn refcount_funcs(&self) -> &'static RefcountFuncs;
}

pub struct Ref<T: Refcount> {
    plumbing: RefImpl,
    no_send: marker::NoSend,
    no_sync: marker::NoSync
}

pub struct SyncRef<T: Refcount + Send + Sync> {
    plumbing: RefImpl
}

impl<T: Refcount> Ref<T> {
    pub fn new(source: &mut T) -> Ref<T> {
        unsafe { make_ref(source as *mut T) }
    }

    pub fn raw_ptr(&self) -> *const T {
        self.plumbing.ptr as gconstpointer as *const T
    }
    pub fn raw_mut_ptr(&self) -> *mut T {
        self.plumbing.ptr as *mut T
    }
}

impl<T: Refcount + Send + Sync> SyncRef<T> {
    pub fn new(source: &mut T) -> SyncRef<T> {
        unsafe { make_sync_ref(source as *mut T) }
    }

    pub fn raw_ptr(&self) -> *const T {
        self.plumbing.ptr as gconstpointer as *const T
    }
    pub fn raw_mut_ptr(&self) -> *mut T {
        self.plumbing.ptr as *mut T
    }
}

pub mod raw {

    use super::{Refcount,RefImpl,Ref,SyncRef};
    use types::gpointer;
    use std::kinds::marker;

    pub unsafe fn ref_from_ptr<T: Refcount>(p: *mut T) -> Ref<T> {
        Ref {
            plumbing: RefImpl::from_ptr(p as gpointer),
            no_send: marker::NoSend,
            no_sync: marker::NoSync
        }
    }

    pub unsafe fn sync_ref_from_ptr<T: Refcount + Send + Sync>(p: *mut T)
                                                             -> SyncRef<T> {
        SyncRef {
            plumbing: RefImpl::from_ptr(p as gpointer)
        }
    }
}

struct RefImpl {
    ptr: gpointer
}

impl RefImpl {
    fn from_ptr(p: gpointer) -> RefImpl {
        RefImpl { ptr: p }
    }

    unsafe fn impl_drop(&mut self, (_, dec_ref): RefcountFuncs) {
        (*dec_ref)(self.ptr);
    }

    unsafe fn impl_clone_from(&mut self, source: &RefImpl,
                              (inc_ref, dec_ref): RefcountFuncs) {
        (*inc_ref)(source.ptr);
        (*dec_ref)(self.ptr);
        self.ptr = source.ptr;
    }
}

unsafe fn make_ref_impl(p: gpointer, (inc_ref, _): RefcountFuncs) -> RefImpl {
    (*inc_ref)(p);
    RefImpl::from_ptr(p)
}

pub unsafe fn make_ref<T: Refcount>(p: *mut T) -> Ref<T> {
    Ref {
        plumbing: make_ref_impl(p as gpointer, *(*p).refcount_funcs()),
        no_send: marker::NoSend,
        no_sync: marker::NoSync
    }
}

pub unsafe fn make_sync_ref<T: Refcount + Send + Sync>(p: *mut T) -> SyncRef<T> {
    SyncRef {
        plumbing: make_ref_impl(p as gpointer, *(*p).refcount_funcs())
    }
}

#[unsafe_destructor]
impl<T: Refcount> Drop for Ref<T> {
    fn drop(&mut self) {
        unsafe {
            let funcs = (*self.raw_ptr()).refcount_funcs();
            self.plumbing.impl_drop(*funcs);
        }
    }
}

#[unsafe_destructor]
impl<T: Refcount + Send + Sync> Drop for SyncRef<T> {
    fn drop(&mut self) {
        unsafe {
            let funcs = (*self.raw_ptr()).refcount_funcs();
            self.plumbing.impl_drop(*funcs);
        }
    }
}

impl<T: Refcount> Clone for Ref<T> {

    fn clone(&self) -> Ref<T> {
        unsafe { make_ref(self.raw_ptr() as *mut T) }
    }

    fn clone_from(&mut self, source: &Ref<T>) {
        unsafe {
            let funcs = (*self.raw_ptr()).refcount_funcs();
            self.plumbing.impl_clone_from(&source.plumbing, *funcs);
        }
    }
}

impl<T: Refcount + Send + Sync> Clone for SyncRef<T> {

    fn clone(&self) -> SyncRef<T> {
        unsafe { make_sync_ref(self.raw_ptr() as *mut T) }
    }

    fn clone_from(&mut self, source: &SyncRef<T>) {
        unsafe {
            let funcs = (*self.raw_ptr()).refcount_funcs();
            self.plumbing.impl_clone_from(&source.plumbing, *funcs);
        }
    }
}

unsafe impl<T> Send for SyncRef<T> where T: Refcount + Send + Sync { }

impl<T> Deref for Ref<T> where T: Refcount {

    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.raw_ptr() }
    }
}

impl<T> Deref for SyncRef<T> where T: Refcount + Send + Sync {

    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.raw_ptr() }
    }
}

impl<T> DerefMut for Ref<T> where T: Refcount {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.raw_mut_ptr() }
    }
}

impl<T> DerefMut for SyncRef<T> where T: Refcount + Send + Sync {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.raw_mut_ptr() }
    }
}
