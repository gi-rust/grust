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
