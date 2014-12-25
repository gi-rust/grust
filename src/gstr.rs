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
use std::mem::transmute;
use std::ptr;
use std::slice;
use std::str;
use util::is_false;

const NUL: u8 = 0;

pub struct GStr {
    ptr: *const gchar,
}

impl GStr {

    pub unsafe fn from_raw_buf(ptr: *mut gchar) -> GStr {
        assert!(ptr.is_not_null());
        GStr { ptr: ptr as *const gchar }
    }

    pub fn parse_as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let r: &'a *const u8 = transmute(&self.ptr);
            slice::from_raw_buf(r, libc::strlen(self.ptr) as uint)
        }
    }

    pub fn parse_as_utf8<'a>(&'a self) -> Option<&'a str> {
        let mut end: *const gchar = ptr::null();
        let valid = unsafe { ffi::g_utf8_validate(self.ptr, -1, &mut end) };
        if is_false(valid) {
            return None;
        }
        unsafe {
            let r: &'a *const u8 = transmute(&self.ptr);
            let bytes = slice::from_raw_buf(r,
                    end as uint - self.ptr as uint);
            Some(str::from_utf8_unchecked(bytes))
        }
    }

    #[inline]
    pub unsafe fn parse_as_utf8_unchecked<'a>(&'a self) -> &'a str {
        str::from_utf8_unchecked(self.parse_as_bytes())
    }
}

impl Drop for GStr {
    fn drop(&mut self) {
        unsafe { ffi::g_free(self.ptr as gpointer) }
    }
}

impl Clone for GStr {
    fn clone(&self) -> GStr {
        unsafe {
            GStr::from_raw_buf(ffi::g_strdup(self.ptr))
        }
    }
}

impl PartialEq for GStr {
    fn eq(&self, other: &GStr) -> bool {
        unsafe { libc::strcmp(self.ptr, other.ptr) == 0 }
    }
}

impl Eq for GStr { }

enum GStrData {
    Static(&'static [u8]),
    Owned(Vec<u8>),
    GLib(GStr)
}

pub struct Utf8Arg {
    data: GStrData
}

fn vec_into_utf8(mut v: Vec<u8>) -> Utf8Arg {
    v.push(NUL);
    Utf8Arg { data: GStrData::Owned(v) }
}

impl Utf8Arg {

    pub fn from_str(s: &str) -> Option<Utf8Arg> {
        let bytes = s.as_bytes();
        if bytes.contains(&NUL) {
            return None;
        }
        Some(vec_into_utf8(bytes.to_vec()))
    }

    pub fn from_static(s: &'static str) -> Utf8Arg {
        let bytes = s.as_bytes();
        if bytes[bytes.len() - 1] != 0 {
            panic!("static string is not null-terminated: \"{}\"", s);
        }
        Utf8Arg { data: GStrData::Static(bytes) }
    }

    pub fn as_ptr(&self) -> *const gchar {
        match self.data {
            GStrData::Static(s) => s.as_ptr() as *const gchar,
            GStrData::Owned(ref v) => v.as_ptr() as *const gchar,
            GStrData::GLib(ref g) => g.ptr
        }
    }
}

pub trait IntoUtf8 {

    fn into_utf8(self) -> Option<Utf8Arg>;

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg;
}

impl<'a> IntoUtf8 for &'a str {

    #[inline]
    fn into_utf8(self) -> Option<Utf8Arg> {
        Utf8Arg::from_str(self)
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        vec_into_utf8(self.as_bytes().to_vec())
    }
}

impl IntoUtf8 for String {

    fn into_utf8(self) -> Option<Utf8Arg> {
        if self.as_bytes().contains(&NUL) {
            None
        } else {
            Some(vec_into_utf8(self.into_bytes()))
        }
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        vec_into_utf8(self.into_bytes())
    }
}

impl IntoUtf8 for GStr {

    fn into_utf8(self) -> Option<Utf8Arg> {
        let valid = unsafe {
            ffi::g_utf8_validate(self.ptr, -1, ptr::null_mut())
        };
        if is_false(valid) {
            return None;
        }
        Some(Utf8Arg { data: GStrData::GLib(self) })
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        Utf8Arg { data: GStrData::GLib(self) }
    }
}
