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

use ffi;
use marker;
use refcount::Refcount;
use refcount::{RefcountFuncs,RefFunc,UnrefFunc};
use refcount::SyncRef;
use types::FALSE;
use wrap;
use wrap::Wrapper;

use std::kinds::marker as std_marker;

#[repr(C)]
pub struct MainContext {
    raw: ffi::GMainContext,
    marker: marker::SyncObjectMarker
}

unsafe impl Wrapper for MainContext {
    type Raw = ffi::GMainContext;
}

impl MainContext {
    pub fn default() -> &'static mut MainContext {
        unsafe {
            wrap::from_raw_mut(ffi::g_main_context_default(), wrap::STATIC)
        }
    }
}

type MainContextRefFunc   = unsafe extern "C" fn(p: *mut ffi::GMainContext) -> *mut ffi::GMainContext;
type MainContextUnrefFunc = unsafe extern "C" fn(p: *mut ffi::GMainContext);

const MAIN_CONTEXT_REF_FUNCS: &'static RefcountFuncs = &(
        &ffi::g_main_context_ref
            as *const MainContextRefFunc as *const RefFunc,
        &ffi::g_main_context_unref
            as *const MainContextUnrefFunc as *const UnrefFunc
    );

impl Refcount for MainContext {
    fn refcount_funcs(&self) -> &'static RefcountFuncs {
        MAIN_CONTEXT_REF_FUNCS
    }
}

#[repr(C)]
pub struct MainLoop {
    raw: ffi::GMainLoop,
    marker: marker::SyncObjectMarker
}

unsafe impl Wrapper for MainLoop {
    type Raw = ffi::GMainLoop;
}

pub struct LoopRunner {
    mainloop: *mut ffi::GMainLoop,

    // Can't send the runner around due to the thread default stuff
    no_send: std_marker::NoSend
}

impl LoopRunner {
    pub fn new() -> LoopRunner {
        unsafe {
            let ctx = ffi::g_main_context_new();
            let mainloop = ffi::g_main_loop_new(ctx, FALSE);
            ffi::g_main_context_unref(ctx);

            LoopRunner { mainloop: mainloop, no_send: std_marker::NoSend }
        }
    }

    pub fn run_after(&self, setup: |SyncRef<MainLoop>|) {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(self.mainloop);
            ffi::g_main_context_push_thread_default(ctx);

            setup(SyncRef::from_raw(self.mainloop));

            ffi::g_main_loop_run(self.mainloop);

            ffi::g_main_context_pop_thread_default(ctx);
        }
    }
}

#[unsafe_destructor]
impl Drop for LoopRunner {
    fn drop(&mut self) {
        unsafe {
            ffi::g_main_loop_unref(self.mainloop);
        }
    }
}

impl MainLoop {

    pub fn get_context(&mut self) -> &mut MainContext {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(&mut self.raw);
            wrap::from_raw_mut(ctx, self)
        }
    }

    pub fn quit(&mut self) {
        unsafe {
            ffi::g_main_loop_quit(&mut self.raw);
        }
    }
}

type MainLoopRefFunc   = unsafe extern "C" fn(p: *mut ffi::GMainLoop) -> *mut ffi::GMainLoop;
type MainLoopUnrefFunc = unsafe extern "C" fn(p: *mut ffi::GMainLoop);

const MAIN_LOOP_REF_FUNCS: &'static RefcountFuncs = &(
        &ffi::g_main_loop_ref
            as *const MainLoopRefFunc as *const RefFunc,
        &ffi::g_main_loop_unref
            as *const MainLoopUnrefFunc as *const UnrefFunc
    );

impl Refcount for MainLoop {
    fn refcount_funcs(&self) -> &'static RefcountFuncs {
        MAIN_LOOP_REF_FUNCS
    }
}
