// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use glib as ffi;
use types::{gchar,gpointer};

use libc;
use std::convert::AsRef;
use std::ffi::{CStr, CString, NulError};
use std::mem;
use std::ops::Deref;
use std::str;

pub struct OwnedGStr {
    ptr: *const gchar,
}

impl OwnedGStr {

    pub unsafe fn from_ptr(ptr: *mut gchar) -> OwnedGStr {
        OwnedGStr { ptr: ptr }
    }
}

impl Deref for OwnedGStr {

    type Target = CStr;

    fn deref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl Drop for OwnedGStr {
    fn drop(&mut self) {
        unsafe { ffi::g_free(self.ptr as gpointer) }
    }
}

impl Clone for OwnedGStr {
    fn clone(&self) -> OwnedGStr {
        unsafe {
            OwnedGStr::from_ptr(ffi::g_strdup(self.ptr))
        }
    }
}

impl PartialEq for OwnedGStr {
    fn eq(&self, other: &OwnedGStr) -> bool {
        unsafe { libc::strcmp(self.ptr, other.ptr) == 0 }
    }
}

impl Eq for OwnedGStr { }

pub struct Utf8 {
    inner: CStr
}

impl Utf8 {

    #[inline]
    pub fn as_ptr(&self) -> *const gchar {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn to_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.inner.to_bytes()) }
    }

    pub fn from_static_str(s: &'static str) -> &'static Utf8 {
        assert!(s.ends_with("\0"),
                "static string is not null-terminated: \"{}\"", s);
        unsafe { Utf8::from_ptr(s.as_ptr() as *const gchar) }
    }

    pub unsafe fn from_ptr<'a>(ptr: *const gchar) -> &'a Utf8 {
        mem::transmute(CStr::from_ptr(ptr))
    }
}

impl AsRef<CStr> for Utf8 {
    #[inline]
    fn as_ref(&self) -> &CStr { &self.inner }
}

pub struct Utf8String {
    inner: CString
}

impl Deref for Utf8String {

    type Target = Utf8;

    fn deref(&self) -> &Utf8 {
        unsafe { Utf8::from_ptr(self.inner.as_ptr()) }
    }
}

impl Utf8String {
    pub fn new<T>(t: T) -> Result<Utf8String, NulError>
        where T: Into<String>
    {
        let c_str = try!(CString::new(t.into()));
        Ok(Utf8String { inner: c_str })
    }
}
