// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2014, 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use refcount::{Refcount, Ref};
use types::{FALSE, TRUE};
use types::{gboolean, gint, gpointer, guint};
use util::{box_free, box_from_pointer, box_into_pointer, into_destroy_notify};
use wrap;
use wrap::Wrapper;

use glib as ffi;
use gobject;
use std::convert;
use std::marker;
use std::mem;

pub const PRIORITY_DEFAULT      : gint = ffi::G_PRIORITY_DEFAULT;
pub const PRIORITY_DEFAULT_IDLE : gint = ffi::G_PRIORITY_DEFAULT_IDLE;
pub const PRIORITY_HIGH         : gint = ffi::G_PRIORITY_HIGH;
pub const PRIORITY_HIGH_IDLE    : gint = ffi::G_PRIORITY_HIGH_IDLE;
pub const PRIORITY_LOW          : gint = ffi::G_PRIORITY_LOW;

pub enum CallbackResult { Remove, Continue }
pub use self::CallbackResult::*;

pub struct RawCallback {
    func: ffi::GSourceFunc,
    data: gpointer,
    destroy: ffi::GDestroyNotify
}

impl Drop for RawCallback {
    fn drop(&mut self) {
        (self.destroy)(self.data);
    }
}

unsafe fn into_source_func(func: unsafe extern "C" fn(gpointer) -> gboolean)
                          -> ffi::GSourceFunc
{
    mem::transmute(func)
}

unsafe extern "C" fn source_func<F>(callback_data: gpointer) -> gboolean
    where F: FnMut() -> CallbackResult
{
    let mut callback: Box<F> = box_from_pointer(callback_data);
    let res = callback();
    mem::forget(callback);
    match res {
        Remove => FALSE,
        Continue => TRUE
    }
}

unsafe extern "C" fn source_once_func<F>(callback_data: gpointer) -> gboolean
    where F: FnOnce()
{
    let mut holder: Box<Option<F>> = box_from_pointer(callback_data);
    let callback = holder.take().expect("a callback closure expected");
    mem::forget(holder);
    callback();
    FALSE
}

pub struct SourceCallback(RawCallback);

impl Into<RawCallback> for SourceCallback {
    #[inline]
    fn into(self) -> RawCallback {
        self.0
    }
}

impl SourceCallback {
    pub fn new<F>(closure: F) -> Self
        where F: Send + 'static, F: FnMut() -> CallbackResult
    {
        let boxed_closure = Box::new(closure);
        SourceCallback(unsafe {
            RawCallback {
                func: into_source_func(source_func::<F>),
                data: box_into_pointer(boxed_closure),
                destroy: into_destroy_notify(box_free::<F>)
            }
        })
    }

    pub fn once<F>(closure: F) -> Self
        where F: Send + 'static, F: FnOnce()
    {
        let holder = Box::new(Some(closure));
        SourceCallback(unsafe {
            RawCallback {
                func: into_source_func(source_once_func::<F>),
                data: box_into_pointer(holder),
                destroy: into_destroy_notify(box_free::<Option<F>>)
            }
        })
    }
}

#[repr(C)]
pub struct MainContext {
    raw: ffi::GMainContext
}

unsafe impl Send for MainContext { }
unsafe impl Sync for MainContext { }
unsafe impl Wrapper for MainContext {
    type Raw = ffi::GMainContext;
}

impl MainContext {
    pub fn default() -> &'static MainContext {
        unsafe {
            wrap::from_raw(ffi::g_main_context_default())
        }
    }

    pub fn invoke(&self, callback: SourceCallback) {
        self.invoke_full(PRIORITY_DEFAULT, callback)
    }

    pub fn invoke_full(&self, priority: gint, callback: SourceCallback) {
        let raw: RawCallback = callback.into();
        unsafe {
            ffi::g_main_context_invoke_full(self.as_mut_ptr(),
                    priority, raw.func, raw.data, Some(raw.destroy));
        }
        mem::forget(raw);
    }
}

impl Refcount for MainContext {

    unsafe fn inc_ref(&self) {
        ffi::g_main_context_ref(self.as_mut_ptr());
    }

    unsafe fn dec_ref(&self) {
        ffi::g_main_context_unref(self.as_mut_ptr());
    }
}

g_impl_boxed_type_for_ref!(MainContext, gobject::g_main_context_get_type);

#[repr(C)]
pub struct Source<Callback = SourceCallback> {
    raw: ffi::GSource,
    phantom_data: marker::PhantomData<Callback>
}

#[repr(C)]
pub struct AttachedSource<Callback> {
    raw: ffi::GSource,
    phantom_data: marker::PhantomData<Callback>
}

unsafe impl<C> Send for Source<C> where C: Into<RawCallback> { }

unsafe impl<C> Send for AttachedSource<C> where C: Into<RawCallback> { }
unsafe impl<C> Sync for AttachedSource<C> where C: Into<RawCallback> { }

macro_rules! common_source_impls {
    ($name:ident) => {
        unsafe impl<C> Wrapper for $name<C> {
            type Raw = ffi::GSource;
        }

        impl<C> Refcount for $name<C> {
            unsafe fn inc_ref(&self) {
                ffi::g_source_ref(self.as_mut_ptr());
            }
            unsafe fn dec_ref(&self) {
                ffi::g_source_unref(self.as_mut_ptr());
            }
        }
    }
}

common_source_impls!(Source);
common_source_impls!(AttachedSource);

impl<C> Source<C> where C: Into<RawCallback> {
    pub fn set_callback(&self, callback: C)
    {
        let raw: RawCallback = callback.into();
        unsafe {
            ffi::g_source_set_callback(self.as_mut_ptr(),
                    raw.func, raw.data, Some(raw.destroy));
        }
        mem::forget(raw);
    }

    pub fn set_priority(&self, priority: gint) {
        unsafe {
            ffi::g_source_set_priority(self.as_mut_ptr(), priority);
        }
    }
}

impl<C> Ref<Source<C>> {
    pub fn attach(self, ctx: &MainContext) -> Ref<AttachedSource<C>> {
        unsafe {
            let source_ptr = self.as_mut_ptr();
            ffi::g_source_attach(source_ptr, ctx.as_mut_ptr());
            mem::forget(self);
            Ref::from_raw(source_ptr)
        }
    }
}

impl<C> AttachedSource<C> {
    #[inline]
    pub fn as_source(&self) -> &Source<C> {
        unsafe { wrap::from_raw(self.as_ptr()) }
    }

    pub fn destroy(&self) {
        unsafe { ffi::g_source_destroy(self.as_mut_ptr()) }
    }
}

impl<C> convert::AsRef<Source<C>> for AttachedSource<C> {
    #[inline]
    fn as_ref(&self) -> &Source<C> {
        self.as_source()
    }
}

pub fn idle_source_new() -> Ref<Source> {
    unsafe {
        let source = ffi::g_idle_source_new();
        Ref::from_raw(source)
    }
}

pub fn timeout_source_new(interval: guint) -> Ref<Source> {
    unsafe {
        let source = ffi::g_timeout_source_new(interval);
        Ref::from_raw(source)
    }
}

pub fn timeout_source_new_seconds(interval: guint) -> Ref<Source> {
    unsafe {
        let source = ffi::g_timeout_source_new_seconds(interval);
        Ref::from_raw(source)
    }
}

#[repr(C)]
pub struct MainLoop {
    raw: ffi::GMainLoop
}

unsafe impl Send for MainLoop { }
unsafe impl Sync for MainLoop { }
unsafe impl Wrapper for MainLoop {
    type Raw = ffi::GMainLoop;
}

pub struct LoopRunner {
    mainloop: *mut ffi::GMainLoop,
}

impl LoopRunner {
    pub fn new() -> LoopRunner {
        unsafe {
            let ctx = ffi::g_main_context_new();
            let mainloop = ffi::g_main_loop_new(ctx, FALSE);
            ffi::g_main_context_unref(ctx);

            LoopRunner { mainloop: mainloop }
        }
    }

    pub fn run_after<F>(&self, setup: F) where F: FnOnce(Ref<MainLoop>) {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(self.mainloop);
            ffi::g_main_context_push_thread_default(ctx);

            setup(Ref::new(wrap::from_raw(self.mainloop)));

            ffi::g_main_loop_run(self.mainloop);

            ffi::g_main_context_pop_thread_default(ctx);
        }
    }
}

impl Drop for LoopRunner {
    fn drop(&mut self) {
        unsafe {
            ffi::g_main_loop_unref(self.mainloop);
        }
    }
}

impl MainLoop {

    pub fn get_context(&self) -> &MainContext {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(self.as_mut_ptr());
            wrap::from_raw(ctx)
        }
    }

    pub fn quit(&self) {
        unsafe {
            ffi::g_main_loop_quit(self.as_mut_ptr());
        }
    }
}

impl Refcount for MainLoop {

    unsafe fn inc_ref(&self) {
        ffi::g_main_loop_ref(self.as_mut_ptr());
    }

    unsafe fn dec_ref(&self) {
        ffi::g_main_loop_unref(self.as_mut_ptr());
    }
}

g_impl_boxed_type_for_ref!(MainLoop, gobject::g_main_loop_get_type);
