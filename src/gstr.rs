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
use util::is_false;

use libc;
use std::error::Error;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

const NUL: u8 = 0;

pub struct GStr {
    ptr: *const gchar,
}

pub unsafe fn parse_as_bytes(raw: & *const gchar) -> &[u8] {
    assert!(raw.is_not_null());
    let r = mem::copy_lifetime(raw, &(*raw as *const u8));
    slice::from_raw_buf(r, libc::strlen(*raw) as uint)
}

#[inline]
pub unsafe fn parse_as_utf8(raw: & *const gchar)
                           -> Result<&str, str::Utf8Error>
{
    str::from_utf8(parse_as_bytes(raw))
}

#[inline]
pub unsafe fn parse_as_utf8_unchecked(raw: & *const gchar) -> &str {
    str::from_utf8_unchecked(parse_as_bytes(raw))
}

impl GStr {

    pub unsafe fn from_raw_buf(ptr: *mut gchar) -> GStr {
        assert!(ptr.is_not_null());
        GStr { ptr: ptr as *const gchar }
    }

    pub fn parse_as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let r = mem::copy_lifetime(self, &(self.ptr as *const u8));
            slice::from_raw_buf(r, libc::strlen(self.ptr) as uint)
        }
    }

    #[inline]
    pub fn parse_as_utf8<'a>(&'a self) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(self.parse_as_bytes())
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

#[deriving(Copy)]
pub enum StrDataError {
    ContainsNul(uint),
    InvalidUtf8(uint)
}

impl Error for StrDataError {

    fn description(&self) -> &str {
        match *self {
            StrDataError::ContainsNul(_)
                => "invalid data for C string: contains a zero byte",
            StrDataError::InvalidUtf8(_)
                => "invalid UTF-8 sequence",
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            StrDataError::ContainsNul(pos)
                => Some(format!("NUL at position {}", pos)),
            StrDataError::InvalidUtf8(pos)
                => Some(format!("Invalid sequence at {}", pos)),
        }
    }
}

impl fmt::Show for StrDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StrDataError::ContainsNul(pos)
                => write!(f, "invalid data for C string: NUL at position {}", pos),
            StrDataError::InvalidUtf8(pos)
                => write!(f, "invalid UTF-8 sequence at position {}", pos),
        }
    }
}

enum GStrData {
    Static(&'static [u8]),
    Owned(Vec<u8>),
    GLib(GStr)
}

impl GStrData {

    fn as_ptr(&self) -> *const gchar {
        match *self {
            GStrData::Static(s) => s.as_ptr() as *const gchar,
            GStrData::Owned(ref v) => v.as_ptr() as *const gchar,
            GStrData::GLib(ref g) => g.ptr
        }
    }

    fn from_static_str(s: &'static str) -> GStrData {
        assert!(s.ends_with("\0"),
                "static string is not null-terminated: \"{}\"", s);
        GStrData::Static(s.as_bytes())
    }

    fn from_static_bytes(bytes: &'static [u8]) -> GStrData {
        assert!(bytes.last() == Some(&NUL),
                "static byte string is not null-terminated: {}", bytes);
        GStrData::Static(bytes)
    }
}

fn vec_into_g_str_data(mut v: Vec<u8>) -> GStrData {
    v.push(NUL);
    GStrData::Owned(v)
}

macro_rules! g_str_data_from_bytes(
    ($inp:expr) => {
        {
            let bytes = $inp;
            if let Some(pos) = bytes.position_elem(&NUL) {
                return Err(StrDataError::ContainsNul(pos));
            }
            vec_into_g_str_data(bytes.to_vec())
        }
    }
);

pub struct GStrArg {
    data: GStrData
}

impl GStrArg {

    #[inline]
    pub fn from_str(s: &str) -> Result<GStrArg, StrDataError> {
        GStrArg::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<GStrArg, StrDataError> {
        Ok(GStrArg { data: g_str_data_from_bytes!(bytes) })
    }

    pub fn from_static_str(s: &'static str) -> GStrArg {
        GStrArg { data: GStrData::from_static_str(s) }
    }

    pub fn from_static_bytes(bytes: &'static [u8]) -> GStrArg {
        GStrArg { data: GStrData::from_static_bytes(bytes) }
    }

    pub fn as_ptr(&self) -> *const gchar {
        self.data.as_ptr()
    }
}

pub struct Utf8Arg {
    data: GStrData
}

impl Utf8Arg {

    pub fn from_str(s: &str) -> Result<Utf8Arg, StrDataError> {
        Ok(Utf8Arg { data: g_str_data_from_bytes!(s.as_bytes()) })
    }

    pub fn from_static_str(s: &'static str) -> Utf8Arg {
        Utf8Arg { data: GStrData::from_static_str(s) }
    }

    pub fn as_ptr(&self) -> *const gchar {
        self.data.as_ptr()
    }
}

pub trait IntoUtf8 {

    fn into_utf8(self) -> Result<Utf8Arg, StrDataError>;

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg;
}

fn vec_into_utf8(v: Vec<u8>) -> Utf8Arg {
    Utf8Arg { data: vec_into_g_str_data(v) }
}

impl<'a> IntoUtf8 for &'a str {

    #[inline]
    fn into_utf8(self) -> Result<Utf8Arg, StrDataError> {
        Utf8Arg::from_str(self)
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        vec_into_utf8(self.as_bytes().to_vec())
    }
}

impl IntoUtf8 for String {

    fn into_utf8(self) -> Result<Utf8Arg, StrDataError> {
        Ok(Utf8Arg { data: g_str_data_from_bytes!(self.into_bytes()) })
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        vec_into_utf8(self.into_bytes())
    }
}

impl IntoUtf8 for GStr {

    fn into_utf8(self) -> Result<Utf8Arg, StrDataError> {
        let mut end: *const gchar = ptr::null_mut();
        let valid = unsafe { ffi::g_utf8_validate(self.ptr, -1, &mut end) };
        if is_false(valid) {
            let pos = end as uint - self.ptr as uint;
            return Err(StrDataError::InvalidUtf8(pos));
        }
        Ok(Utf8Arg { data: GStrData::GLib(self) })
    }

    unsafe fn into_utf8_unchecked(self) -> Utf8Arg {
        Utf8Arg { data: GStrData::GLib(self) }
    }
}
