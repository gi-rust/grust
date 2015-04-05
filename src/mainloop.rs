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

use refcount::{Refcount, Ref};
use types::FALSE;
use wrap;
use wrap::Wrapper;

use glib as ffi;
use gobject;

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
