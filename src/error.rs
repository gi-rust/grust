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

use std::default::Default;
use std::error::Error as ErrorTrait;
use std::mem;
use std::ptr;

pub mod raw {

    use ffi;
    use types::{gint,gchar};
    use std::kinds::marker;

    #[repr(C)]
    pub struct GError {
        pub domain: ffi::GQuark,
        pub code: gint,
        pub message: *const gchar,
        no_copy: marker::NoCopy
    }
}

pub struct Error {
    ptr: *mut raw::GError
}

pub fn unset() -> Error {
    Error { ptr: ptr::null_mut() }
}

#[derive(Show)]
pub enum Match<T> {
    NotInDomain,
    Known(T),
    Unknown(int)
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
            unset()
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
    fn default() -> Error { unset() }
}

impl Error {
    pub unsafe fn slot_ptr(&mut self) -> *mut *mut raw::GError {
        &mut self.ptr as *mut *mut raw::GError
    }

    pub fn is_set(&self) -> bool { !self.ptr.is_null() }

    pub fn key(&self) -> (Quark, int) {
        if self.ptr.is_null() {
            panic!("use of an unset GError pointer slot");
        }
        unsafe {
            let raw = &*self.ptr;
            (Quark::new(raw.domain), raw.code as int)
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
            os = unsafe { gstr::parse_as_utf8(&raw.message).ok() };
        }
        if os.is_none() {
            let domain = unsafe { Quark::new(raw.domain) };
            os = domain.to_str().ok().map(|s| {
                unsafe { mem::copy_lifetime(self, s) }
            });
        }
        if let Some(s) = os {
            s
        } else {
            "[non-UTF-8 message]"
        }
    }

    fn detail(&self) -> Option<String> {
        if !self.ptr.is_null() {
            let msg = unsafe { &(*self.ptr).message };
            if !msg.is_null() {
                let msg_bytes = unsafe { gstr::parse_as_bytes(msg) };
                return Some(String::from_utf8_lossy(msg_bytes).into_owned());
            }
        }
        None
    }
}
