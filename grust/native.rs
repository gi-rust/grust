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
use mainloop::{MainContext,TaskLoop};
use refcount::{Ref,Refcount};
use refcount::{RefcountFuncs,RefFunc,UnrefFunc};
use refcount::raw::ref_from_ptr;
use types::FALSE;

#[repr(C)]
pub struct MainLoop;

impl MainLoop {
    pub fn new(ctx: &mut MainContext) -> Ref<MainLoop> {
        unsafe {
            ref_from_ptr(ffi::g_main_loop_new(ctx, FALSE))
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

impl TaskLoop for MainLoop {

    fn run_after(&mut self, setup: |&mut MainContext|) {
        unsafe {
            let ctx = ffi::g_main_loop_get_context(self);

            ffi::g_main_context_push_thread_default(ctx);

            setup(&mut *ctx);

            ffi::g_main_loop_run(self);

            ffi::g_main_context_pop_thread_default(ctx);
        }
    }

    fn quit(&mut self) {
        unsafe {
            ffi::g_main_loop_quit(self);
        }
    }
}
