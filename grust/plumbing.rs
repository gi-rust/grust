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
use ll::*;
pub use types::{GMainContext, GMainLoop, GObject, GTypeInstance};

pub struct Object {
    priv raw_obj: *GObject,
    priv context: *GMainContext
}

pub unsafe fn get_object(obj: *GObject, ctx: *GMainContext) -> Object {
    Object { raw_obj: obj, context: ctx }
}

pub unsafe fn take_object(obj: *GObject, ctx: *GMainContext) -> Object {
    debug!("task %d: taking object %? (ref context %?)",
           *task::get_task(), obj, ctx);
    Object { raw_obj: obj, context: g_main_context_ref(ctx) }
}

impl Object {
    pub unsafe fn raw(&self) -> *GObject { self.raw_obj }

    pub unsafe fn type_instance(&self) -> *GTypeInstance {
        self.raw_obj as *GTypeInstance
    }

    pub unsafe fn context(&self) -> *GMainContext { self.context }

    pub unsafe fn inc_ref(&self) {
        debug!("task %d: ref object %? (ref context %?)",
               *task::get_task(), self.raw_obj, self.context);
        g_object_ref(self.raw_obj as *());
        g_main_context_ref(self.context);
    }

    pub unsafe fn dec_ref(&self) {
        debug!("task %d: unref object %? (unref context %?)",
               *task::get_task(), self.raw_obj, self.context);
        g_object_unref(self.raw_obj as *());
        g_main_context_unref(self.context);
    }
}

struct CallbackData {
    callback: *(),
    context: *GMainContext
}

extern fn grust_call_cb(data: *(), ctx: *GMainContext) {
    unsafe {
        let func = *(data as *&fn(*GMainContext));
        func(ctx);
    } 
}

pub unsafe fn call(ctx: *GMainContext, func: &fn(*GMainContext)) {
    if !(grustna_call(grust_call_cb,
                      ptr::to_unsafe_ptr(&func) as *(),
                      ctx)
         as bool) {
        fail!(~"call failure");
    }
}
