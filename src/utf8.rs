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
use types::{gchar,gsize};
use types::{gpointer};

use libc;
use libc::c_char;
use std::c_str::{CString,ToCStr};
use std::kinds::marker;
use std::mem::transmute;
use std::ptr::copy_nonoverlapping_memory;
use std::slice;
use std::str;
use std::string;

pub struct UTF8Chars<'a> {
    data: *const gchar,
    lifetime: marker::ContravariantLifetime<'a>
}

impl<'a> UTF8Chars<'a> {
    pub unsafe fn wrap(ptr: *const gchar) -> UTF8Chars<'a> {
        UTF8Chars { data: ptr, lifetime: marker::ContravariantLifetime }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            string::raw::from_buf(self.data as *const u8)
        }
    }
}

#[inline]
fn utf8_cont_bits(p: *const gchar) -> u32 {
    let byte = unsafe { *(p as *const u8) };
    debug_assert!((byte & 0xC0) == 0x80);
    (byte & 0x3F) as u32
}

impl<'a> Iterator<char> for UTF8Chars<'a> {
    fn next(&mut self) -> Option<char> {
        let p = self.data;
        let first_byte = unsafe { *(p as *const u8) };
        if first_byte == 0 {
            return None;
        }
        unsafe {
            let mut wc: u32;
            if first_byte < 0xC0 {
                debug_assert!(first_byte < 0x80);
                wc = first_byte as u32;
                self.data = p.offset(1);
            } else {
                let c1 = utf8_cont_bits(p.offset(1));
                if first_byte < 0xE0 {
                    wc = ((first_byte & 0x1F) as u32 << 6) | c1;
                    debug_assert!(wc >= 0x80);
                    self.data = p.offset(2);
                } else {
                    let c2 = utf8_cont_bits(p.offset(2));
                    if first_byte < 0xF0 {
                        wc = ((first_byte & 0x0F) as u32 << 12)
                             | (c1 << 6) | c2;
                        debug_assert!(wc >= 0x0800);
                        debug_assert!((wc & 0xF800) != 0xD800);
                        self.data = p.offset(3);
                    } else {
                        debug_assert!(first_byte <= 0xF4);
                        let c3 = utf8_cont_bits(p.offset(3));
                        wc = ((first_byte & 0x07) as u32 << 18)
                             | (c1 << 12) | (c2 << 6) | c3;
                        debug_assert!(wc >= 0x010000 && wc <= 0x10FFFF);
                        self.data = p.offset(4);
                    }
                }
            }
            Some(transmute(wc))
        }
    }
}

unsafe fn dup_to_c_str(source: *const c_char, len: uint) -> CString {
    let copy = libc::malloc(len as libc::size_t + 1) as *mut c_char;
    if copy.is_null() {
        ::alloc::oom();
    }
    copy_nonoverlapping_memory(copy, source, len + 1);
    CString::new(copy as *const c_char, true)
}

impl<'a> ToCStr for UTF8Chars<'a> {

    #[inline]
    fn to_c_str(&self) -> CString {
        unsafe { self.to_c_str_unchecked() }
    }

    unsafe fn to_c_str_unchecked(&self) -> CString {
        dup_to_c_str(self.data, libc::strlen(self.data) as uint)
    }

    fn with_c_str<T>(&self, f: |*const c_char| -> T) -> T {
        f(self.data)
    }

    unsafe fn with_c_str_unchecked<T>(&self, f: |*const c_char| -> T) -> T {
        f(self.data)
    }
}

pub struct UTF8Buf {
    data: *const gchar,
}

impl UTF8Buf {

    pub unsafe fn wrap(data: *mut gchar) -> UTF8Buf {
        UTF8Buf { data: data as *const gchar }
    }

    pub fn chars<'a>(&'a self) -> UTF8Chars<'a> {
        unsafe { UTF8Chars::wrap(self.data) }
    }

    pub fn into_collection(self) -> UTF8Str {
        unsafe {
            let len = libc::strlen(self.data);
            UTF8Str { buf: transmute(self), len: len as uint }
        }
    }

    #[inline]
    pub fn to_string(&self) -> String { self.chars().to_string() }

    #[inline]
    pub fn into_string(self) -> String { self.to_string() }

    pub fn is_empty(&self) -> bool { unsafe { *self.data == 0 } }
}

impl Drop for UTF8Buf {
    fn drop(&mut self) {
        unsafe { ffi::g_free(self.data as gpointer) }
    }
}

impl Clone for UTF8Buf {
    fn clone(&self) -> UTF8Buf {
        unsafe {
            UTF8Buf::wrap(ffi::g_strdup(self.data))
        }
    }
}

impl ToCStr for UTF8Buf {

    #[inline]
    fn to_c_str(&self) -> CString {
        self.chars().to_c_str()
    }

    #[inline]
    unsafe fn to_c_str_unchecked(&self) -> CString {
        self.chars().to_c_str_unchecked()
    }

    fn with_c_str<T>(&self, f: |*const c_char| -> T) -> T {
        f(self.data)
    }

    unsafe fn with_c_str_unchecked<T>(&self, f: |*const c_char| -> T) -> T {
        f(self.data)
    }
}

pub struct UTF8Str {
    buf: UTF8Buf,
    len: uint
}

const NUL: gchar = 0;

impl UTF8Str {

    pub unsafe fn wrap(data: *mut gchar, len: uint) -> UTF8Str {
        UTF8Str { buf: UTF8Buf::wrap(data), len: len }
    }

    pub fn chars<'a>(&'a self) -> UTF8Chars<'a> { self.buf.chars() }

    pub fn len(&self) -> uint { return self.len }

    pub fn is_empty(&self) -> bool { return self.len == 0 }

    pub fn to_string(&self) -> String {
        unsafe {
            string::raw::from_buf_len(self.buf.data as *const u8, self.len)
        }
    }
}

impl Clone for UTF8Str {
    fn clone(&self) -> UTF8Str {
        unsafe {
            let copy = ffi::g_malloc((self.len + 1) as gsize) as *mut gchar;
            copy_nonoverlapping_memory(copy,
                    self.buf.data as *const gchar, self.len);
            *copy.offset(self.len as int) = NUL;
            UTF8Str::wrap(copy, self.len)
        }
    }
}

impl Str for UTF8Str {
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

impl StrAllocating for UTF8Str {
    fn into_string(self) -> String { self.to_string() }
}

impl ToCStr for UTF8Str {

    fn to_c_str(&self) -> CString {
        unsafe { self.to_c_str_unchecked() }
    }

    unsafe fn to_c_str_unchecked(&self) -> CString {
        dup_to_c_str(self.buf.data, self.len)
    }

    fn with_c_str<T>(&self, f: |*const c_char| -> T) -> T {
        self.buf.with_c_str(f)
    }

    unsafe fn with_c_str_unchecked<T>(&self, f: |*const c_char| -> T) -> T {
        self.buf.with_c_str_unchecked(f)
    }
}

impl PartialEq for UTF8Str {
    fn eq(&self, other: &UTF8Str) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for UTF8Str { }

impl<S: Str> Equiv<S> for UTF8Str {
    fn equiv(&self, other: &S) -> bool {
        self.as_slice() == other.as_slice()
    }
}

pub trait WithUTF8 for Sized? {
    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T;
}

impl<'a> WithUTF8 for UTF8Chars<'a> {
    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        f(self.data)
    }
}

impl WithUTF8 for UTF8Buf {

    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        f(self.data as *const gchar)
    }
}

impl WithUTF8 for UTF8Str {

    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        self.buf.with_utf8_c_str(f)
    }
}

impl WithUTF8 for str {
    #[inline]
    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        self.with_c_str(f)
    }
}

impl<'a, Sized? T: WithUTF8> WithUTF8 for &'a T {
    #[inline]
    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        (**self).with_utf8_c_str(f)
    }
}

impl WithUTF8 for String {
    #[inline]
    fn with_utf8_c_str<T>(&self, f: |*const gchar| -> T) -> T {
        self.with_c_str(f)
    }
}
