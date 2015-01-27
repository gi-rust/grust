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

pub unsafe fn parse_as_bytes<'a, T: ?Sized>(raw: *const gchar,
                                            life_anchor: &'a T)
                                           -> &'a [u8]
{
    assert!(!raw.is_null());
    let r = mem::copy_lifetime(life_anchor, &(raw as *const u8));
    slice::from_raw_buf(r, libc::strlen(raw) as usize)
}

#[inline]
pub unsafe fn parse_as_utf8<'a, T: ?Sized>(raw: *const gchar,
                                           life_anchor: &'a T)
                                          -> Result<&'a str, str::Utf8Error>
{
    str::from_utf8(parse_as_bytes(raw, life_anchor))
}

impl OwnedGStr {

    pub unsafe fn from_raw(ptr: *mut gchar) -> OwnedGStr {
        assert!(!ptr.is_null());
        OwnedGStr { ptr: ptr as *const gchar }
    }
}

impl Deref for OwnedGStr {

    type Target = GStr;

    fn deref(&self) -> &GStr {
        unsafe { g_str_from_ptr_internal(self.ptr as *const u8, self) }
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
            OwnedGStr::from_raw(ffi::g_strdup(self.ptr))
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

    pub fn parse_as_bytes(&self) -> &[u8] {
        unsafe {
            let r = mem::copy_lifetime(self, &(self.as_ptr() as *const u8));
            slice::from_raw_buf(r, libc::strlen(self.as_ptr()) as usize)
        }
    }

    #[inline]
    pub fn parse_as_utf8(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.parse_as_bytes())
    }

    #[inline]
    pub unsafe fn parse_as_utf8_unchecked(&self) -> &str {
        str::from_utf8_unchecked(self.parse_as_bytes())
    }

    pub fn from_static_bytes(bytes: &'static [u8]) -> &'static GStr {
        assert!(bytes.last() == Some(&NUL),
                "static byte string is not null-terminated: \"{}\"",
                escape_bytestring(bytes));
        unsafe { g_str_from_ptr_internal(bytes.as_ptr(), bytes) }
    }

    pub unsafe fn from_raw<'a, T: ?Sized>(ptr: *const gchar,
                                          life_anchor: &'a T)
                                         -> &'a GStr
    {
        assert!(!ptr.is_null());
        g_str_from_ptr_internal(ptr as *const u8, life_anchor)
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
    pub fn parse_as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.gstr.parse_as_bytes()) }
    }

    pub fn from_static_str(s: &'static str) -> &'static Utf8 {
        assert!(s.ends_with("\0"),
                "static string is not null-terminated: \"{}\"", s);
        unsafe { utf8_from_ptr_internal(s.as_ptr(), s) }
    }

    pub unsafe fn from_raw<'a, T: ?Sized>(ptr: *const gchar,
                                          life_anchor: &'a T)
                                         -> &'a Utf8
    {
        assert!(!ptr.is_null());
        utf8_from_ptr_internal(ptr as *const u8, life_anchor)
    }
}

unsafe fn g_str_from_ptr_internal<'a, T: ?Sized>(ptr: *const u8,
                                                 life_anchor: &'a T)
                                                -> &'a GStr
{
    mem::copy_lifetime(life_anchor, &*(ptr as *const GStr))
}

unsafe fn utf8_from_ptr_internal<'a, T: ?Sized>(ptr: *const u8,
                                                life_anchor: &'a T)
                                               -> &'a Utf8
{
    mem::copy_lifetime(life_anchor, &*(ptr as *const Utf8))
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
        unsafe { g_str_from_ptr_internal(self.data.as_ptr(), self) }
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
        unsafe { utf8_from_ptr_internal(self.inner.data.as_ptr(), self) }
    }
}

fn utf8_wrap_g_str_result<E>(res: Result<GStrBuf, E>) -> Result<Utf8Buf, E> {
    res.map(|buf| {
        Utf8Buf { inner: buf }
    })
}

impl Utf8Buf {

    pub fn from_str(s: &str) -> Result<Utf8Buf, NulError> {
        utf8_wrap_g_str_result(GStrBuf::from_bytes(s.as_bytes()))
    }

    pub fn from_string(s: String) -> Result<Utf8Buf, IntoGStrError> {
        utf8_wrap_g_str_result(GStrBuf::from_vec(s.into_bytes()))
    }
}
