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

use grust::gstr::{GStr, IntoUtf8, StrDataError, Utf8Arg};

use grust::types::gchar;

use glib::raw::g_strdup;

use std::ptr;
use libc;

static TEST_CSTR: &'static str = "¡Hola, amigos!\0";
static TEST_STR:  &'static str = "¡Hola, amigos!";

fn new_g_str(source: &str) -> GStr {
    assert!(source.ends_with("\0"));
    unsafe {
        let p = source.as_ptr();
        GStr::from_raw_buf(g_strdup(p as *const gchar))
    }
}

fn new_g_str_from_bytes(source: &[u8]) -> GStr {
    assert!(source[source.len() - 1] == 0);
    unsafe {
        let p = source.as_ptr();
        GStr::from_raw_buf(g_strdup(p as *const gchar))
    }
}

fn g_str_equal(p1: *const gchar, p2: *const gchar) -> bool {
    let cmp_res = unsafe { libc::strcmp(p1, p2) };
    cmp_res == 0
}

#[test]
fn test_parse_as_utf8() {
    let str = new_g_str(TEST_CSTR);
    let res = str.parse_as_utf8();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), TEST_STR);
}

#[test]
fn test_parse_as_utf8_invalid() {
    let s = new_g_str_from_bytes(b"a\x80\0");
    let res = s.parse_as_utf8();
    assert!(res.is_err());
    match res {
        Err(_) => {}
        _ => unreachable!()
    }
}

#[test]
fn test_parse_as_bytes() {
    let str = new_g_str_from_bytes(b"a\x80\0");
    let bytes = str.parse_as_bytes();
    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], b'a');
    assert_eq!(bytes[1], b'\x80');
}

#[test]
#[should_fail]
fn test_str_from_null() {
    let _ = unsafe { GStr::from_raw_buf(ptr::null_mut()) };
}

#[test]
fn test_g_str_clone() {
    let str1 = new_g_str(TEST_CSTR);
    let str2 = str1.clone();
    let s = str2.parse_as_utf8().unwrap();
    assert_eq!(s, String::from_str(TEST_STR));
}

#[test]
fn test_g_str_eq() {
    let s1 = new_g_str(TEST_CSTR);
    let s2 = new_g_str(TEST_CSTR);
    assert!(s1 == s2);
}

#[test]
fn test_g_str_ne() {
    let s1 = new_g_str(TEST_CSTR);
    let s2 = new_g_str("This is not the string you are looking for\0");
    assert!(s1 != s2);
}

#[test]
fn test_g_str_into_inner() {
    let s = new_g_str(TEST_CSTR);
    let p = unsafe { s.into_inner() };
    assert!(g_str_equal(p, TEST_CSTR.as_ptr() as *const gchar));

    // Wrap the pointer again so it does not get leaked
    let _ = unsafe { GStr::from_raw_buf(p) };
}

#[test]
fn test_utf8_arg_from_static_str() {
    let s = Utf8Arg::from_static_str(TEST_CSTR);
    assert_eq!(s.as_ptr(), TEST_CSTR.as_ptr() as *const gchar);
}

#[test]
fn test_utf8_arg_from_str() {
    let s = String::from_str(TEST_STR);
    let c = Utf8Arg::from_str(s.as_slice()).unwrap();
    assert!(g_str_equal(c.as_ptr(), TEST_CSTR.as_ptr() as *const gchar));
}

#[test]
fn test_utf8_arg_from_str_error() {
    let res = Utf8Arg::from_str("got\0nul");
    match res {
        Err(StrDataError::ContainsNul(pos)) => assert_eq!(pos, 3),
        _ => unreachable!()
    }
}

#[test]
fn test_string_into_utf8() {
    let s = String::from_str(TEST_STR);
    let c = s.into_utf8().unwrap();
    assert!(g_str_equal(c.as_ptr(), TEST_CSTR.as_ptr() as *const gchar));
}

#[test]
fn test_g_str_into_utf8() {
    let gs = new_g_str(TEST_CSTR);
    let arg = gs.into_utf8().unwrap();
    assert!(g_str_equal(arg.as_ptr(), TEST_CSTR.as_ptr() as *const gchar));
}

#[test]
fn test_g_str_into_utf8_error() {
    let gs = new_g_str_from_bytes(b"a\x80\0");
    let res = gs.into_utf8();
    match res {
        Err(StrDataError::InvalidUtf8(pos)) => assert_eq!(pos, 1),
        _ => unreachable!()
    }
}
