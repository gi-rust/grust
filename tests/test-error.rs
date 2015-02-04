// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

#![feature(core)]

#[macro_use]
extern crate grust;

extern crate "glib-2_0-sys" as glib;

use grust::error;
use grust::error::{Error, Domain, DomainError};
use grust::gstr::GStrBuf;
use grust::quark::Quark;
use grust::types::gint;

use std::error::Error as ErrorTrait;
use std::error::FromError;

const NON_UTF8: &'static [u8] = b"U can't parse this.\x9c Hammer time!";
const NON_UTF8_LOSSY: &'static str = "U can't parse this.\u{FFFD} Hammer time!";
const NON_UTF8_ESCAPED: &'static str = r#"U can\'t parse this.\x9c Hammer time!"#;

const A_FOO: gint = 1;
const A_BAR: gint = 2;
const B_BAZ: gint = 1;

const UNKNOWN_CODE: gint = 1337;

#[derive(Copy, Debug, Eq, PartialEq, FromPrimitive)]
enum AError {
    Foo = A_FOO as isize,
    Bar = A_BAR as isize,
}

#[derive(Copy, Debug, Eq, PartialEq, FromPrimitive)]
enum BError {
    Baz = B_BAZ as isize
}

impl Domain for AError {

    fn domain() -> Quark {
        g_static_quark!(b"a-error\0")
    }

    fn name(&self) -> &'static str {
        match *self {
            AError::Foo => "foo",
            AError::Bar => "bar"
        }
    }
}

impl Domain for BError {

    fn domain() -> Quark {
        g_static_quark!(b"b-error\0")
    }

    fn name(&self) -> &'static str {
        match *self {
            BError::Baz => "baz"
        }
    }
}

fn new_error<T>(code: gint, message: &[u8]) -> Error where T: Domain {
    let domain = error::domain::<T>();
    let msg_buf = GStrBuf::from_bytes(message).unwrap();
    unsafe {
        let raw = glib::g_error_new_literal(
            domain.to_raw(), code, msg_buf.as_ptr());
        Error::from_raw(raw)
    }
}

fn new_domain_error<T>(code: gint, message: &[u8]) -> DomainError<T>
    where T: Domain
{
    new_error::<T>(code, message).into_domain().unwrap()
}

#[test]
fn test_domain() {
    let domain = error::domain::<AError>();
    assert_eq!(domain, <AError as Domain>::domain());
    assert_eq!(domain.to_bytes(), b"a-error");
    let domain = error::domain::<BError>();
    assert_eq!(domain, <BError as Domain>::domain());
    assert_eq!(domain.to_bytes(), b"b-error");
}

#[test]
fn test_error_domain() {
    let err = new_error::<AError>(A_FOO, b"test error");
    assert_eq!(err.domain(), error::domain::<AError>());
}

#[test]
fn test_error_matches() {
    let err = new_error::<AError>(A_FOO, b"test error");
    assert!(err.matches(AError::Foo));
    assert!(!err.matches(AError::Bar));
    assert!(!err.matches(BError::Baz));
}

#[test]
fn test_error_key() {
    let err_foo1 = new_error::<AError>(A_FOO, b"test error");
    let err_foo2 = new_error::<AError>(A_FOO, b"same error, different message");
    assert_eq!(err_foo1.key(), err_foo2.key());
    let err_bar = new_error::<AError>(A_BAR, b"test error");
    assert!(err_foo1.key() != err_bar.key());
    let err_baz = new_error::<BError>(B_BAZ, b"test error");
    assert!(err_foo1.key() != err_baz.key());
}

#[test]
fn test_error_in_domain() {
    let err = new_error::<AError>(A_FOO, b"test error");
    assert!(err.in_domain::<AError>());
    assert!(!err.in_domain::<BError>());
}

#[test]
fn test_error_into_domain() {
    let err = new_error::<AError>(A_FOO, b"test error");
    let res = err.into_domain::<AError>();
    let a_err = res.unwrap();
    assert_eq!(a_err.code(), error::Code::Known(AError::Foo));
}

#[test]
fn test_domain_error_code() {
    let err = new_domain_error::<AError>(A_FOO, b"test error");
    let code = err.code();
    match code {
        error::Code::Known(c) => assert_eq!(c, AError::Foo),
        error::Code::Unknown(_) => unreachable!()
    }
    let known = code.known();
    assert_eq!(known, Some(AError::Foo));

    let err = new_domain_error::<AError>(UNKNOWN_CODE, b"unknown error");
    let code = err.code();
    match code {
        error::Code::Known(_) => unreachable!(),
        error::Code::Unknown(int_code) => assert_eq!(int_code, UNKNOWN_CODE)
    }
    let known = code.known();
    assert!(known.is_none());
}

#[test]
fn test_error_description() {
    let message = "test error";
    let err = new_error::<AError>(A_FOO, message.as_bytes());
    assert_eq!(err.description(), message);
    let err = new_error::<AError>(A_FOO, NON_UTF8);
    assert_eq!(err.description(), "a-error");
}

#[test]
fn test_domain_error_description() {
    let message = "test error";
    let err = new_domain_error::<AError>(A_FOO, message.as_bytes());
    assert_eq!(err.description(), message);
    let err = new_domain_error::<AError>(A_FOO, NON_UTF8);
    assert_eq!(err.description(), "foo");
}

#[test]
fn test_error_display() {
    let message = "test error";
    let err = new_error::<AError>(A_FOO, message.as_bytes());
    let s = format!("{}", err);
    assert_eq!(s.as_slice(), message);
    let err = new_error::<AError>(A_FOO, NON_UTF8);
    let s = format!("{}", err);
    assert_eq!(s.as_slice(), NON_UTF8_LOSSY)
}

#[test]
fn test_domain_error_display() {
    let message = "test error";
    let err = new_domain_error::<AError>(A_FOO, message.as_bytes());
    let s = format!("{}", err);
    assert_eq!(s.as_slice(), message);
    let err = new_domain_error::<AError>(A_FOO, NON_UTF8);
    let s = format!("{}", err);
    assert_eq!(s.as_slice(), NON_UTF8_LOSSY)
}

macro_rules! assert_contains_or_dump {
    ($inp:expr, $needle:expr) => {
        assert!($inp.contains($needle),
                "unexpected `Debug` formatting: `{}`", $inp)
    }
}

macro_rules! assert_contains {
    ($inp:expr, $str:expr) => (
        assert_contains_or_dump!($inp, $str)
    );
    ($inp:expr, $fmt:expr, $($arg:expr),*) => (
        assert_contains_or_dump!($inp, format!($fmt, $($arg),*).as_slice())
    )
}

#[test]
fn test_error_debug() {
    let message = "test error";
    let err = new_error::<AError>(A_FOO, message.as_bytes());
    let s = format!("{:?}", err);
    let (domain, code) = err.key();
    let domain_str = domain.to_g_str().to_utf8().unwrap();
    assert_contains!(s, "GError");
    assert_contains!(s, "{}", domain_str);
    assert_contains!(s, "{}", code);
    assert_contains!(s, r#""{}""#, message);

    let err = new_error::<BError>(B_BAZ, NON_UTF8);
    let s = format!("{:?}", err);
    let (domain, code) = err.key();
    let domain_str = domain.to_g_str().to_utf8().unwrap();
    assert_contains!(s, "GError");
    assert_contains!(s, "{}", domain_str);
    assert_contains!(s, "{}", code);
    assert_contains!(s, r#""{}""#, NON_UTF8_ESCAPED);
}

#[test]
fn test_domain_error_debug() {
    let message = "test error";
    let err = new_domain_error::<AError>(A_FOO, message.as_bytes());
    let s = format!("{:?}", err);
    let domain = error::domain::<AError>();
    let domain_str = domain.to_g_str().to_utf8().unwrap();
    let code = err.code().known().unwrap();
    assert_contains!(s, "GError");
    assert_contains!(s, "{}", domain_str);
    assert_contains!(s, "{}", code.name());
    assert_contains!(s, r#""{}""#, message);

    let err = new_domain_error::<BError>(B_BAZ, NON_UTF8);
    let s = format!("{:?}", err);
    let domain = error::domain::<BError>();
    let domain_str = domain.to_g_str().to_utf8().unwrap();
    let code = err.code().known().unwrap();
    assert_contains!(s, "GError");
    assert_contains!(s, "{}", domain_str);
    assert_contains!(s, "{}", code.name());
    assert_contains!(s, r#""{}""#, NON_UTF8_ESCAPED);

    let err = new_domain_error::<AError>(UNKNOWN_CODE, b"unknown error");
    let s = format!("{:?}", err);
    let domain = error::domain::<AError>();
    let domain_str = domain.to_g_str().to_utf8().unwrap();
    assert_contains!(s, "GError");
    assert_contains!(s, "{}", domain_str);
    assert_contains!(s, "{}", UNKNOWN_CODE);
}

#[test]
fn test_error_from_domain_error() {
    let message = "test error";
    let domain_err = new_domain_error::<AError>(A_FOO, message.as_bytes());
    let err: Error = FromError::from_error(domain_err);
    let (domain, code) = err.key();
    assert_eq!(domain, error::domain::<AError>());
    assert_eq!(code, A_FOO);
    assert_eq!(err.description(), message);
}

#[test]
fn test_error_clone() {
    let message = "test error";
    let err = new_error::<AError>(A_FOO, message.as_bytes());
    let err2 = err.clone();
    assert_eq!(err2.description(), message);
}

#[test]
fn test_domain_error_clone() {
    let message = "test error";
    let err = new_domain_error::<AError>(A_FOO, message.as_bytes());
    let err2 = err.clone();
    assert_eq!(err2.description(), message);
}

#[test]
fn test_error_match_macro() {
    let err = new_error::<AError>(A_FOO, b"test error");
    g_error_match! {
        (err) {
            other any_err => {
                // Test that the error value can be moved
                any_err.into_domain::<AError>().unwrap();
            }
        }
    }
    let err = new_error::<AError>(A_FOO, b"test error");
    g_error_match! {
        (err) {
            (a_err: DomainError<AError>) => {
                assert_eq!(a_err.code(), error::Code::Known(AError::Foo));
            },
            (_b_err: DomainError<BError>) => unreachable!(),
            other _err => unreachable!()
        }
    }
}
