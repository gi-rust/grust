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

use gobject::raw::GObject;
use gobject::raw::symbols::{g_object_ref,g_object_unref};

pub struct Object<R> {
    priv wrapped: *R
}

pub unsafe fn wrap_object<R>(obj: *R) -> Object<R> {
    Object { wrapped: obj }
}

impl<R> Object<R> {
    pub unsafe fn unwrap(&self) -> *R { self.wrapped }

    pub unsafe fn get_g_object(&self) -> *GObject {
        self.wrapped as *GObject
    }
}

#[unsafe_destructor]
impl<R> Drop for Object<R> {
    fn finalize(&self) {
        unsafe {
            g_object_unref(self.get_g_object());
        }
    }
}

impl<R> Clone for Object<R> {
    fn clone(&self) -> Object<R> {
        unsafe {
            g_object_ref(self.get_g_object());
            wrap_object(self.wrapped)
        }
    }
}
