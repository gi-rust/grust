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
use std::default::Default;
use std::ptr;

pub mod raw {

    use ffi;
    use types::{gint,gchar};

    #[repr(C)]
    pub struct GError {
        domain: ffi::GQuark,
        code: gint,
        message: *const gchar
    }
}

pub struct Error {
    ptr: *mut raw::GError
}

pub fn init() -> Error {
    Error { ptr: ptr::null_mut() }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe { ffi::g_error_free(self.ptr); }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        if self.ptr.is_null() {
            init()
        } else {
            unsafe {
                Error { ptr: ffi::g_error_copy(self.ptr as *const raw::GError) }
            }
        }
    }
}

impl Default for Error {
    fn default() -> Error { init() }
}

impl Error {
    pub unsafe fn slot_ptr(&mut self) -> *mut *mut raw::GError {
        &mut self.ptr as *mut *mut raw::GError
    }

    pub fn is_set(&self) -> bool { self.ptr.is_not_null() }
}
