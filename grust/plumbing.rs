/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use na::{grustna_call_on_stack,grustna_call_off_stack};
use glib::{g_main_context_ref, g_main_context_unref};
use gobject::{g_object_ref,g_object_unref};

pub trait GMainContext { }

pub struct Object<R> {
    priv wrapped: *R,
    priv context: *GMainContext
}

pub unsafe fn take_object<R>(obj: *R, ctx: *GMainContext) -> Object<R> {
    debug!("task %d: taking object %? (ref context %?)",
           *task::get_task(), obj, ctx);
    Object { wrapped: obj, context: g_main_context_ref(ctx) }
}

impl<R> Object<R> {
    pub unsafe fn raw(&self) -> *R { self.wrapped }
    pub unsafe fn context(&self) -> *GMainContext { self.context }
}

extern fn call_on_stack_cb(data: *(), ctx: *GMainContext) {
    unsafe {
        let func = *(data as *&fn(*GMainContext));
        func(ctx);
    } 
}

extern fn call_off_stack_cb(data: *(), ctx: *GMainContext) {
    unsafe {
        let func = *(data as *~fn(*GMainContext));
        func(ctx);
    } 
}

pub unsafe fn call_on_stack(ctx: *GMainContext, func: &fn(*GMainContext))
        -> bool {
    return grustna_call_on_stack(call_on_stack_cb,
                                 ptr::to_unsafe_ptr(&func) as *(),
                                 ctx)
           as bool;
}

pub unsafe fn call_off_stack(ctx: *GMainContext, func: ~fn(*GMainContext)) {
    if !(grustna_call_off_stack(call_off_stack_cb,
                                ptr::to_unsafe_ptr(&func) as *(),
                                ctx)
         as bool) {
        fail!(~"off-stack call failure");
    }
}

#[unsafe_destructor]
impl<R> Drop for Object<R> {
    fn finalize(&self) {
        unsafe {
            debug!("task %d: finalize (unref context %?) unref object %?",
                   *task::get_task(), self.context, self.wrapped);
            g_main_context_unref(self.context);
            g_object_unref(self.wrapped as *());
        }
    }
}

impl<R> Clone for Object<R> {
    fn clone(&self) -> Object<R> {
        unsafe {
            debug!("task %d: clone ref object %?", *task::get_task(), self.wrapped);
            g_object_ref(self.wrapped as *());
            take_object(self.wrapped, self.context)
        }
    }
}
