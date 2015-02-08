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
use util::escape_bytestring;

use libc;
use std::error::Error;
use std::fmt;
use std::marker;
use std::mem;
use std::ops::Deref;
use std::slice;
use std::str;

const NUL: u8 = 0;

pub struct OwnedGStr {
    ptr: *const gchar,
}

impl OwnedGStr {

    pub unsafe fn from_ptr(ptr: *mut gchar) -> OwnedGStr {
        OwnedGStr { ptr: ptr as *const gchar }
    }
}

impl Deref for OwnedGStr {

    type Target = GStr;

    fn deref(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.ptr) }
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

#[derive(Copy)]
pub struct NulError {
    pub position: usize
}

impl Error for NulError {

    fn description(&self) -> &str {
        "invalid data for C string: contains a NUL byte"
    }
}

impl fmt::Display for NulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid data for C string: NUL at position {}",
               self.position)
    }
}

impl fmt::Debug for NulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct IntoGStrError {
    cause: NulError,
    bytes: Vec<u8>
}

impl IntoGStrError {

    pub fn nul_error(&self) -> &NulError {
        &self.cause
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl Error for IntoGStrError {

    fn description(&self) -> &str {
        self.cause.description()
    }
}

impl fmt::Display for IntoGStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl fmt::Debug for IntoGStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cause)
    }
}

#[repr(C)]
pub struct GStr {
    head: gchar,
    marker: marker::NoCopy
}

#[repr(C)]
pub struct Utf8 {
    gstr: GStr
}

impl GStr {

    #[inline]
    pub fn as_ptr(&self) -> *const gchar {
        &self.head as *const gchar
    }

    pub fn to_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr();
        unsafe {
            slice::from_raw_parts(ptr as *const u8, libc::strlen(ptr) as usize)
        }
    }

    #[inline]
    pub fn to_utf8(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.to_bytes())
    }

    #[inline]
    pub unsafe fn to_utf8_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.to_bytes())
    }

    pub fn from_static_bytes(bytes: &'static [u8]) -> &'static GStr {
        assert!(bytes.last() == Some(&NUL),
                "static byte string is not null-terminated: \"{}\"",
                escape_bytestring(bytes));
        unsafe { GStr::from_ptr(bytes.as_ptr() as *const gchar) }
    }

    pub unsafe fn from_ptr<'a>(ptr: *const gchar) -> &'a GStr {
        mem::transmute(&*(ptr as *const GStr))
    }
}

impl Utf8 {

    #[inline]
    pub fn as_ptr(&self) -> *const gchar {
        self.gstr.as_ptr()
    }

    #[inline]
    pub fn as_g_str(&self) -> &GStr {
        &self.gstr
    }

    #[inline]
    pub fn to_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.gstr.to_bytes()) }
    }

    pub fn from_static_str(s: &'static str) -> &'static Utf8 {
        assert!(s.ends_with("\0"),
                "static string is not null-terminated: \"{}\"", s);
        unsafe { Utf8::from_ptr(s.as_ptr() as *const gchar) }
    }

    pub unsafe fn from_ptr<'a>(ptr: *const gchar) -> &'a Utf8 {
        mem::transmute(&*(ptr as *const Utf8))
    }
}

fn vec_into_g_str_buf(mut v: Vec<u8>) -> GStrBuf {
    v.push(NUL);
    GStrBuf { data: v }
}

pub struct GStrBuf {
    data: Vec<u8>
}

impl Deref for GStrBuf {

    type Target = GStr;

    fn deref(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.data.as_ptr() as *const gchar) }
    }
}

impl GStrBuf {

    #[inline]
    pub fn from_str(s: &str) -> Result<GStrBuf, NulError> {
        GStrBuf::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<GStrBuf, NulError> {
        if let Some(pos) = bytes.position_elem(&NUL) {
            return Err(NulError { position: pos });
        }
        Ok(vec_into_g_str_buf(bytes.to_vec()))
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<GStrBuf, IntoGStrError> {
        if let Some(pos) = vec.position_elem(&NUL) {
            return Err(IntoGStrError {
                cause: NulError { position: pos },
                bytes: vec
            });
        }
        Ok(vec_into_g_str_buf(vec))
    }
}

pub struct Utf8Buf {
    inner: GStrBuf
}

impl Deref for Utf8Buf {

    type Target = Utf8;

    fn deref(&self) -> &Utf8 {
        unsafe { Utf8::from_ptr(self.inner.data.as_ptr() as *const gchar) }
    }
}

unsafe fn utf8_wrap_g_str_result<E>(res: Result<GStrBuf, E>)
                                   -> Result<Utf8Buf, E>
{
    res.map(|buf| {
        Utf8Buf { inner: buf }
    })
}

impl Utf8Buf {

    pub fn from_str(s: &str) -> Result<Utf8Buf, NulError> {
        let g_str_res = GStrBuf::from_bytes(s.as_bytes());
        unsafe { utf8_wrap_g_str_result(g_str_res) }
    }

    pub fn from_string(s: String) -> Result<Utf8Buf, IntoGStrError> {
        let g_str_res = GStrBuf::from_vec(s.into_bytes());
        unsafe { utf8_wrap_g_str_result(g_str_res) }
    }
}
