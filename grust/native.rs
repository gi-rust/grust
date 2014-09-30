/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use ffi;
use mainloop::MainContext;
use plumbing::Threadsafe;
use refcount::{Refcount,SyncRef};
use refcount::make_sync_ref;
use refcount::{RefcountFuncs,RefFunc,UnrefFunc};
use types::FALSE;

use std::kinds::marker;

#[repr(C)]
pub struct MainLoop;

pub struct LoopRunner {
    l: *mut MainLoop,

    // Can't send the runner around due to the thread default stuff
    no_send: marker::NoSend
}

impl LoopRunner {
    pub fn new() -> LoopRunner {
        unsafe {
            let ctx = ffi::g_main_context_new();

            let l = ffi::g_main_loop_new(ctx, FALSE);

            ffi::g_main_context_push_thread_default(ctx);
            ffi::g_main_context_unref(ctx);

            LoopRunner { l: l, no_send: marker::NoSend }
        }
    }

    pub fn ref_loop(&self) -> SyncRef<MainLoop> {
        unsafe { make_sync_ref(self.l) }
    }

    pub fn run(&self) {
        unsafe {
            ffi::g_main_loop_run(self.l);
        }
    }
}

#[unsafe_destructor]
impl Drop for LoopRunner {
    fn drop(&mut self) {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(self.l);
            ffi::g_main_context_pop_thread_default(ctx);
            ffi::g_main_loop_unref(self.l);
        }
    }
}

impl MainLoop {

    pub fn get_context<'a>(&'a mut self) -> &'a mut MainContext {
        unsafe {
            &mut *ffi::g_main_loop_get_context(self)
        }
    }

    pub fn quit(&mut self) {
        unsafe {
            ffi::g_main_loop_quit(self);
        }
    }
}

type MainLoopRefFunc   = unsafe extern "C" fn(p: *mut ffi::GMainLoop) -> *mut ffi::GMainLoop;
type MainLoopUnrefFunc = unsafe extern "C" fn(p: *mut ffi::GMainLoop);

static main_loop_ref_funcs: RefcountFuncs = (
        &ffi::g_main_loop_ref
            as *const MainLoopRefFunc as *const RefFunc,
        &ffi::g_main_loop_unref
            as *const MainLoopUnrefFunc as *const UnrefFunc
    );

impl Refcount for MainLoop {
    fn refcount_funcs(&self) -> &'static RefcountFuncs {
        &main_loop_ref_funcs
    }
}

impl Threadsafe for MainLoop { }
