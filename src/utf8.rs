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

use alloc::libc_heap::malloc_raw;
use libc;
use libc::c_char;
use std::c_str::{CString,ToCStr};
use std::mem::transmute;
use std::ptr::copy_nonoverlapping_memory;
use std::slice;
use std::str;
use std::string;

pub struct Utf8Buf {
    data: *mut gchar,
}

impl Utf8Buf {

    pub unsafe fn new(data: *mut gchar) -> Utf8Buf {
        Utf8Buf { data: data }
    }

    pub fn into_collection(self) -> Utf8Str {
        unsafe {
            let len = libc::strlen(self.data as *const c_char);
            Utf8Str { buf: transmute(self), len: len as uint }
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            string::raw::from_buf(self.data as *const gchar as *const u8)
        }
    }

    #[inline]
    pub fn to_owned(&self) -> String { self.to_string() }

    #[inline]
    pub fn into_string(self) -> String { self.to_string() }

    #[inline]
    pub fn into_owned(self) -> String { self.to_string() }

    #[inline]
    pub fn is_empty(&self) -> bool { unsafe { *self.data == 0 } }
}

impl Drop for Utf8Buf {
    fn drop(&mut self) {
        unsafe { ffi::g_free(self.data as gpointer) }
    }
}

impl Clone for Utf8Buf {
    fn clone(&self) -> Utf8Buf {
        unsafe {
            Utf8Buf::new(ffi::g_strdup(self.data as *const gchar))
        }
    }
}

unsafe fn dup_to_c_str(source: *const c_char, len: uint) -> CString {
    let copy = malloc_raw(len + 1) as *mut c_char;
    copy_nonoverlapping_memory(copy, source, len + 1);
    CString::new(copy as *const c_char, true)
}

impl ToCStr for Utf8Buf {

    fn to_c_str(&self) -> CString {
        unsafe { self.to_c_str_unchecked() }
    }

    unsafe fn to_c_str_unchecked(&self) -> CString {
        let src = self.data as *const c_char;
        let len = libc::strlen(src) as uint;
        dup_to_c_str(src, len)
    }

    fn with_c_str<T>(&self, f: |*const i8| -> T) -> T {
        f(self.data as *const i8)
    }

    unsafe fn with_c_str_unchecked<T>(&self, f: |*const i8| -> T) -> T {
        f(self.data as *const i8)
    }
}

#[deriving(Clone)]
pub struct Utf8Str {
    buf: Utf8Buf,
    len: uint
}

impl Str for Utf8Str {
    fn as_slice<'a>(&'a self) -> &'a str {
        unsafe {
            slice::raw::buf_as_slice(
                self.buf.data as *const u8,
                self.len,
                |bytes| {
                    let s = str::raw::from_utf8(bytes);
                    transmute(s)
                })
        }
    }
}

impl StrAllocating for Utf8Str {
    fn into_string(self) -> String { self.buf.to_string() }
}

impl Collection for Utf8Str {
    fn len(&self) -> uint { return self.len }
    fn is_empty(&self) -> bool { return self.len == 0 }
}

impl ToCStr for Utf8Str {

    fn to_c_str(&self) -> CString { self.buf.to_c_str() }

    unsafe fn to_c_str_unchecked(&self) -> CString {
        dup_to_c_str(self.buf.data as *const c_char, self.len)
    }

    fn with_c_str<T>(&self, f: |*const i8| -> T) -> T {
        self.buf.with_c_str(f)
    }

    unsafe fn with_c_str_unchecked<T>(&self, f: |*const i8| -> T) -> T {
        self.buf.with_c_str_unchecked(f)
    }
}

impl PartialEq for Utf8Str {
    fn eq(&self, other: &Utf8Str) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Utf8Str { }

impl<S: Str> Equiv<S> for Utf8Str {
    fn equiv(&self, other: &S) -> bool {
        self.as_slice() == other.as_slice()
    }
}
