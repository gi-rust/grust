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
use gstr;
use quark::Quark;
use types::gint;

use std::default::Default;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::mem;
use std::ptr;
use std::str;

pub mod raw {
    use ffi;

    pub type GError = ffi::GError;
}

pub struct Error {
    ptr: *mut raw::GError
}

pub fn none() -> Error {
    Error { ptr: ptr::null_mut() }
}

#[derive(Show)]
pub enum Match<T> {
    NotInDomain,
    Known(T),
    Unknown(gint)
}

impl<T> PartialEq for Match<T> where T: PartialEq {
    fn eq(&self, other: &Match<T>) -> bool {
        match (self, other) {
            (&Match::Known(ref a), &Match::Known(ref b)) => *a == *b,
            (&Match::Unknown(a), &Match::Unknown(b)) => a == b,
            _ => false
        }
    }
}

unsafe impl Send for Error { }

impl Drop for Error {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::g_error_free(self.ptr); }
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        if self.ptr.is_null() {
            none()
        } else {
            Error {
                ptr: unsafe {
                    ffi::g_error_copy(self.ptr as *const raw::GError)
                }
            }
        }
    }
}

impl Default for Error {
    #[inline]
    fn default() -> Error { none() }
}

impl Error {
    pub unsafe fn slot_ptr(&mut self) -> *mut *mut raw::GError {
        &mut self.ptr as *mut *mut raw::GError
    }

    pub fn is_set(&self) -> bool { !self.ptr.is_null() }

    pub fn key(&self) -> (Quark, gint) {
        if self.ptr.is_null() {
            panic!("use of an unset GError pointer slot");
        }
        unsafe {
            let raw = &*self.ptr;
            (Quark::new(raw.domain), raw.code)
        }
    }
}

impl ErrorTrait for Error {
    fn description<'a>(&'a self) -> &'a str {
        if self.ptr.is_null() {
            return "no error";
        }
        let mut os: Option<&'a str> = None;
        let raw = unsafe { &*self.ptr };
        if !raw.message.is_null() {
            os = unsafe { gstr::parse_as_utf8(raw.message, self).ok() };
        }
        if os.is_none() {
            let domain = unsafe { Quark::new(raw.domain) };
            os = str::from_utf8(domain.as_bytes()).ok().map(|s| {
                unsafe { mem::copy_lifetime(self, s) }
            });
        }
        if let Some(s) = os {
            s
        } else {
            "[non-UTF-8 message]"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.ptr.is_null() {
            write!(f, "no error")
        } else {
            let msg = unsafe {
                gstr::parse_as_bytes((*self.ptr).message, self)
            };
            write!(f, "{}", String::from_utf8_lossy(msg))
        }
    }
}
