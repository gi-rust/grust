// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2014, 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use boxed::BoxedType;
use enumeration;
use enumeration::IntrospectedEnum;
use gtype::GType;
use quark::Quark;
use types::{gint, gpointer};
use util::escape_bytestring;

use glib as ffi;
use gobject;

use std::any::Any;
use std::error::Error as ErrorTrait;
use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::str;

pub struct Error {
    ptr: *mut ffi::GError
}

pub struct DomainError<T> {
    inner: Error,
    marker: PhantomData<T>
}

#[derive(Copy, Clone, Debug)]
pub enum Code<T> {
    Known(T),
    Unknown(gint)
}

pub trait Domain : IntrospectedEnum + Any {
    fn domain() -> Quark;
}

pub fn domain<T>() -> Quark where T: Domain {
    <T as Domain>::domain()
}

unsafe impl Send for Error { }

unsafe impl<T> Send for DomainError<T> { }

impl Drop for Error {
    fn drop(&mut self) {
        unsafe { ffi::g_error_free(self.ptr); }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        Error {
            ptr: unsafe { ffi::g_error_copy(self.ptr) }
        }
    }
}

impl<T> Clone for DomainError<T> {
    fn clone(&self) -> DomainError<T> {
        DomainError { inner: self.inner.clone(), marker: PhantomData }
    }
}

impl BoxedType for Error {

    fn get_type() -> GType {
        unsafe { GType::from_raw(gobject::g_error_get_type()) }
    }

    unsafe fn from_ptr(ptr: gpointer) -> Error {
        Error { ptr: ptr as *mut ffi::GError }
    }

    unsafe fn into_ptr(self) -> gpointer {
        let ptr = self.ptr;
        mem::forget(self);
        ptr as gpointer
    }
}

impl Error {

    pub unsafe fn from_raw(ptr: *mut ffi::GError) -> Error {
        assert!(!ptr.is_null(), "GError pointer is not set");
        Error { ptr: ptr }
    }

    pub fn domain(&self) -> Quark {
        unsafe { Quark::from_raw((*self.ptr).domain) }
    }

    pub fn matches<T>(&self, code: T) -> bool where T: Domain + PartialEq {
        if self.domain() == domain::<T>() {
            let own_code_raw = unsafe { (*self.ptr).code };
            if let Ok(own_code) = enumeration::from_int::<T>(own_code_raw) {
                return own_code == code;
            }
        }
        false
    }

    pub fn key(&self) -> (Quark, gint) {
        unsafe {
            let raw = &*self.ptr;
            (Quark::from_raw(raw.domain), raw.code)
        }
    }

    pub fn in_domain<T>(&self) -> bool where T: Domain {
        self.domain() == domain::<T>()
    }

    pub fn into_domain<T>(self) -> Result<DomainError<T>, Error>
        where T: Domain
    {
        if self.in_domain::<T>() {
            Ok(DomainError { inner: self, marker: PhantomData })
        } else {
            Err(self)
        }
    }

    fn message(&self) -> Option<&str> {
        let message = unsafe { CStr::from_ptr((*self.ptr).message) };
        str::from_utf8(message.to_bytes()).ok()
    }

    fn message_bytes(&self) -> &[u8] {
        let message = unsafe { CStr::from_ptr((*self.ptr).message) };
        message.to_bytes()
    }
}

impl<T> DomainError<T> where T: IntrospectedEnum {
    pub fn code(&self) -> Code<T> {
        let code = unsafe { (*self.inner.ptr).code };
        match enumeration::from_int(code) {
            Ok(domain_code) => Code::Known(domain_code),
            Err(_)          => Code::Unknown(code)
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        if let Some(s) = self.message() {
            return s;
        }
        match str::from_utf8(self.domain().to_bytes()) {
            Ok(s)  => s,
            Err(_) => "GError (message and domain are not represented)"
        }
    }
}

impl<T> ErrorTrait for DomainError<T> where T: Domain {
    fn description(&self) -> &str {
        if let Some(s) = self.inner.message() {
            return s;
        }
        if let Code::Known(code) = self.code() {
            return code.name();
        }
        match str::from_utf8(self.inner.domain().to_bytes()) {
            Ok(s)  => s,
            Err(_) => "GError (message and domain are not represented; unknown code)"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.message_bytes();
        write!(f, "{}", String::from_utf8_lossy(bytes))
    }
}

impl<T> fmt::Display for DomainError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.inner.message_bytes();
        write!(f, "{}", String::from_utf8_lossy(bytes))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (domain, code) = self.key();
        let message = escape_bytestring(self.message_bytes());
        write!(f,
               r#"GError {{ domain: {:?}, code: {}, message: "{}" }}"#,
               domain, code, message)
    }
}

impl<T> fmt::Debug for DomainError<T> where T: Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let domain = self.inner.domain();
        let message = escape_bytestring(self.inner.message_bytes());
        match self.code() {
            Code::Known(code) => {
                write!(f,
                       r#"GError {{ domain: {:?}, code: {:?}, message: "{}" }}"#,
                       domain, code.name(), message)
            }
            Code::Unknown(int_code) => {
                write!(f,
                       r#"GError {{ domain: {:?}, code: {}, message: "{}" }}"#,
                       domain, int_code, message)
            }
        }
    }
}

impl<T> From<DomainError<T>> for Error {
    fn from(err: DomainError<T>) -> Error { err.inner }
}

impl<T> Code<T> {

    pub fn known(self) -> Option<T> {
        match self {
            Code::Known(code) => Some(code),
            Code::Unknown(_) => None
        }
    }
}

impl<T> PartialEq for Code<T> where T: PartialEq {
    fn eq(&self, other: &Code<T>) -> bool {
        match (self, other) {
            (&Code::Known(ref a), &Code::Known(ref b)) => *a == *b,
            (&Code::Unknown(a), &Code::Unknown(b)) => a == b,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glib as ffi;
    use quark::Quark;
    use types::gint;
    use std::ffi::CString;

    fn new_error(domain: Quark, code: gint, message: &[u8]) -> Error {
        let msg_buf = CString::new(message).unwrap();
        unsafe {
            let raw = ffi::g_error_new_literal(
                domain.to_raw(), code, msg_buf.as_ptr());
            Error::from_raw(raw)
        }
    }

    #[test]
    fn message() {
        let domain = Quark::from_static_str("foo\0");
        let message = "test message";
        let err = new_error(domain, 1, message.as_bytes());
        assert_eq!(err.message().unwrap(), message);
        let err = new_error(domain, 1, b"U can't parse this.\x9C Hammer time!");
        assert!(err.message().is_none());
    }

    #[test]
    fn message_bytes() {
        let domain = Quark::from_static_str("foo\0");
        let message = b"U can't parse this.\x9C Hammer time!";
        let err = new_error(domain, 1, message);
        assert_eq!(err.message_bytes(), &message[..]);
    }
}
