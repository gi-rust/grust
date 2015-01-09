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
use wrap;
use wrap::Wrapper;

use std::marker;
use std::mem;
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

unsafe impl<T> Send for SyncRef<T> where T: Refcount + Send + Sync { }
unsafe impl<T> Sync for SyncRef<T> where T: Refcount + Send + Sync { }

impl<T> Ref<T> where T: Refcount {

    pub fn new(source: &mut T) -> Ref<T> {
        new_ref(source)
    }
}

impl<T> Ref<T> where T: Refcount + Wrapper {

    pub unsafe fn from_raw(ptr: *mut <T as Wrapper>::Raw) -> Ref<T> {
        Ref::new(wrap::from_raw_mut(ptr, &ptr))
    }
}

impl<T> SyncRef<T> where T: Refcount + Send + Sync {

    pub fn new(source: &mut T) -> SyncRef<T> {
        new_sync_ref(source)
    }
}

impl<T> SyncRef<T> where T: Refcount + Wrapper + Send + Sync {

    pub unsafe fn from_raw(ptr: *mut <T as Wrapper>::Raw) -> SyncRef<T> {
        SyncRef::new(wrap::from_raw_mut(ptr, &ptr))
    }
}

struct RefImpl {
    ptr: gpointer
}

impl RefImpl {

    unsafe fn impl_drop(&mut self, (_, dec_ref): RefcountFuncs) {
        (*dec_ref)(self.ptr);
    }

    unsafe fn impl_clone_from(&mut self, source: &RefImpl,
                              (inc_ref, dec_ref): RefcountFuncs)
    {
        if self.ptr == source.ptr {
            return;
        }
        (*inc_ref)(source.ptr);
        (*dec_ref)(self.ptr);
        self.ptr = source.ptr;
    }
}

unsafe fn new_ref_impl(p: gpointer, (inc_ref, _): RefcountFuncs) -> RefImpl {
    (*inc_ref)(p);
    RefImpl { ptr: p }
}

fn new_ref<T>(source: &T) -> Ref<T>
    where T: Refcount
{
    let p = source as *const T as *mut T as gpointer;
    let funcs = source.refcount_funcs();
    Ref {
        plumbing: unsafe { new_ref_impl(p, *funcs) },
        no_send: marker::NoSend,
        no_sync: marker::NoSync
    }
}

fn new_sync_ref<T>(source: &T) -> SyncRef<T>
    where T: Refcount + Send + Sync
{
    let p = source as *const T as *mut T as gpointer;
    let funcs = source.refcount_funcs();
    SyncRef {
        plumbing: unsafe { new_ref_impl(p, *funcs) }
    }
}

#[unsafe_destructor]
impl<T> Drop for Ref<T> where T: Refcount {
    fn drop(&mut self) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_drop(*funcs);
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for SyncRef<T> where T: Refcount + Send + Sync {
    fn drop(&mut self) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_drop(*funcs);
        }
    }
}

impl<T: Refcount> Clone for Ref<T> {

    fn clone(&self) -> Ref<T> {
        new_ref(self.deref())
    }

    fn clone_from(&mut self, source: &Ref<T>) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_clone_from(&source.plumbing, *funcs);
        }
    }
}

impl<T: Refcount + Send + Sync> Clone for SyncRef<T> {

    fn clone(&self) -> SyncRef<T> {
        new_sync_ref(self.deref())
    }

    fn clone_from(&mut self, source: &SyncRef<T>) {
        let funcs = self.refcount_funcs();
        unsafe {
            self.plumbing.impl_clone_from(&source.plumbing, *funcs);
        }
    }
}

impl<T> Deref for Ref<T> where T: Refcount {

    type Target = T;

    fn deref(&self) -> &T {
        let p = self.plumbing.ptr as *const T;
        unsafe { mem::copy_lifetime(self, &*p) }
    }
}

impl<T> Deref for SyncRef<T> where T: Refcount + Send + Sync {

    type Target = T;

    fn deref(&self) -> &T {
        let p = self.plumbing.ptr as *const T;
        unsafe { mem::copy_lifetime(self, &*p) }
    }
}

impl<T> DerefMut for Ref<T> where T: Refcount {

    fn deref_mut(&mut self) -> &mut T {
        let p = self.plumbing.ptr as *mut T;
        unsafe { mem::copy_mut_lifetime(self, &mut *p) }
    }
}

impl<T> DerefMut for SyncRef<T> where T: Refcount + Send + Sync {

    fn deref_mut(&mut self) -> &mut T {
        let p = self.plumbing.ptr as *mut T;
        unsafe { mem::copy_mut_lifetime(self, &mut *p) }
    }
}
