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

use ffi;
use native;
use refcount::{Refcount};
use refcount::{RefcountFuncs,RefFunc,UnrefFunc};

#[repr(C)]
pub struct MainContext;

pub type MainLoop = native::MainLoop;

type MainContextRefFunc   = unsafe extern "C" fn(p: *mut ffi::GMainContext) -> *mut ffi::GMainContext;
type MainContextUnrefFunc = unsafe extern "C" fn(p: *mut ffi::GMainContext);

static main_context_ref_funcs: RefcountFuncs = (
        &ffi::g_main_context_ref
            as *const MainContextRefFunc as *const RefFunc,
        &ffi::g_main_context_unref
            as *const MainContextUnrefFunc as *const UnrefFunc
    );

impl Refcount for MainContext {
    fn refcount_funcs(&self) -> &'static RefcountFuncs {
        &main_context_ref_funcs
    }
}

pub trait TaskLoop {
    fn run_after(&mut self, setup: |&mut MainContext|);
    fn quit(&mut self);
}
