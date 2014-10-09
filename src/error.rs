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
use types::{gsize,gssize};
use utf8::UTF8Str;

use std::default::Default;
use std::ptr;
use std::slice;
use std::str;

use libc;

pub mod raw {

    use ffi;
    use types::{gint,gchar};

    #[repr(C)]
    pub struct GError {
        pub domain: ffi::GQuark,
        pub code: gint,
        pub message: *const gchar
    }
}

pub struct Error {
    ptr: *mut raw::GError
}

pub fn unset() -> Error {
    Error { ptr: ptr::null_mut() }
}

impl Drop for Error {
    fn drop(&mut self) {
        if self.ptr.is_not_null() {
            unsafe { ffi::g_error_free(self.ptr); }
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        if self.ptr.is_null() {
            unset()
        } else {
            unsafe {
                Error { ptr: ffi::g_error_copy(self.ptr as *const raw::GError) }
            }
        }
    }
}

impl Default for Error {
    fn default() -> Error { unset() }
}

impl Error {
    pub unsafe fn slot_ptr(&mut self) -> *mut *mut raw::GError {
        &mut self.ptr as *mut *mut raw::GError
    }

    pub fn is_set(&self) -> bool { self.ptr.is_not_null() }

    pub fn message(&self) -> String {

        if self.ptr.is_null() {
            return String::from_str("no error");
        }

        // GError messages may come in any shape or form, but the best guesses
        // at the encoding would be: 1) UTF-8; 2) the locale encoding.

        unsafe {
            let raw_msg = (*self.ptr).message;
            assert!(raw_msg.is_not_null());
            let len = libc::strlen(raw_msg) as uint;

            match slice::raw::buf_as_slice(raw_msg as *const u8, len,
                |b| {
                    str::from_utf8(b).map(String::from_str)
                }) {
                Some(s) => { return s; }
                None    => {}
            }

            let mut bytes_read: gsize = 0;
            let mut bytes_conv: gsize = 0;
            let conv_msg = ffi::g_locale_to_utf8(raw_msg, len as gssize,
                            &mut bytes_read as *mut gsize,
                            &mut bytes_conv as *mut gsize,
                            ptr::null_mut());
            if conv_msg.is_not_null() {
                let str = UTF8Str::wrap(conv_msg, bytes_conv as uint);
                if bytes_read as uint == len {
                    return str.to_string();
                }
            }

            // As the last resort, try to salvage what we can
            slice::raw::buf_as_slice(raw_msg as *const u8, len,
                |b| { String::from_utf8_lossy(b).into_string() })
        }
    }
}
