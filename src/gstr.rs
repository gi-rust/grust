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

pub struct UTF8In {
    data: GStrData
}

fn vec_into_utf8(mut v: Vec<u8>) -> UTF8In {
    v.push(NUL);
    UTF8In { data: GStrData::Owned(v) }
}

impl UTF8In {

    pub fn from_str(s: &str) -> Option<UTF8In> {
        let bytes = s.as_bytes();
        if bytes.contains(&NUL) {
            return None;
        }
        Some(vec_into_utf8(bytes.to_vec()))
    }

    pub fn from_static(s: &'static str) -> UTF8In {
        let bytes = s.as_bytes();
        if bytes[bytes.len() - 1] != 0 {
            panic!("static string is not null-terminated: \"{}\"", s);
        }
        UTF8In { data: GStrData::Static(bytes) }
    }

    pub fn as_ptr(&self) -> *const gchar {
        match self.data {
            GStrData::Static(s) => s.as_ptr() as *const gchar,
            GStrData::Owned(ref v) => v.as_ptr() as *const gchar,
            GStrData::GLib(ref g) => g.ptr
        }
    }
}

pub trait IntoUTF8 {

    fn into_utf8(self) -> Option<UTF8In>;

    unsafe fn into_utf8_unchecked(self) -> UTF8In;
}

impl<'a> IntoUTF8 for &'a str {

    fn into_utf8(self) -> Option<UTF8In> {
        let bytes = self.as_bytes();
        if bytes.contains(&NUL) {
            None
        } else {
            Some(vec_into_utf8(bytes.to_vec()))
        }
    }

    unsafe fn into_utf8_unchecked(self) -> UTF8In {
        vec_into_utf8(self.as_bytes().to_vec())
    }
}

impl IntoUTF8 for String {

    fn into_utf8(self) -> Option<UTF8In> {
        if self.as_bytes().contains(&NUL) {
            None
        } else {
            Some(vec_into_utf8(self.into_bytes()))
        }
    }

    unsafe fn into_utf8_unchecked(self) -> UTF8In {
        vec_into_utf8(self.into_bytes())
    }
}

impl IntoUTF8 for GStr {

    fn into_utf8(self) -> Option<UTF8In> {
        let valid = unsafe {
            ffi::g_utf8_validate(self.ptr, -1, ptr::null_mut())
        };
        if is_false(valid) {
            return None;
        }
        Some(UTF8In { data: GStrData::GLib(self) })
    }

    unsafe fn into_utf8_unchecked(self) -> UTF8In {
        UTF8In { data: GStrData::GLib(self) }
    }
}
