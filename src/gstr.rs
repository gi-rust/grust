// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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
use types::{gchar,gpointer};

use libc;
use libc::{c_char,size_t};
use std::mem::transmute;
use std::slice;
use std::str;
use std::string;

pub struct Utf8 {
    data: *mut gchar,
}

impl Utf8 {
    pub unsafe fn new(data: *mut gchar) -> Utf8 {
        Utf8 { data: data }
    }

    pub fn into_string(self) -> String {
        unsafe {
            string::raw::from_buf(self.data as *const gchar as *const u8)
        }
    }
}

impl Drop for Utf8 {
    fn drop(&mut self) {
        unsafe {
            ffi::g_free(self.data as gpointer);
        }
    }
}

impl Clone for Utf8 {
    fn clone(&self) -> Utf8 {
        unsafe {
            Utf8::new(ffi::g_strdup(self.data as *const gchar))
        }
    }
}

impl Str for Utf8 {
    fn as_slice<'a>(&'a self) -> &'a str {
        unsafe {
            let len = libc::strlen(self.data as *const c_char);
            slice::raw::buf_as_slice(
                self.data as *const u8,
                len as uint,
                |bytes| {
                    let s = str::raw::from_utf8(bytes);
                    transmute(s)
                })
        }
    }
}

impl StrAllocating for Utf8 {
    fn into_string(self) -> String {
        unsafe {
            string::raw::from_buf(self.data as *const gchar as *const u8)
        }
    }
}

impl PartialEq for Utf8 {
    fn eq(&self, other: &Utf8) -> bool {
        unsafe {
            libc::strcmp(self.data as *const gchar,
                         other.data as *const gchar) == 0
        }
    }

    fn ne(&self, other: &Utf8) -> bool {
        unsafe {
            libc::strcmp(self.data as *const c_char,
                         other.data as *const c_char) != 0
        }
    }
}

impl Eq for Utf8 { }

impl<S: Str> Equiv<S> for Utf8 {
    fn equiv(&self, other: &S) -> bool {
        let os = other.as_slice();
        let len = os.len();
        unsafe {
            libc::strncmp(self.data as *const c_char,
                          os.as_ptr() as *const c_char,
                          len as size_t) == 0
            && *self.data.offset(len as int) == 0
        }
    }
}
