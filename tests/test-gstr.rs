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

#[macro_use]
extern crate grust;

extern crate glib_2_0_sys as glib;
extern crate libc;

use grust::gstr;
use grust::gstr::{Utf8String, OwnedGStr};

use grust::types::gchar;

use glib::g_strdup;

use std::str;

static TEST_CSTR: &'static str = "¡Hola, amigos!\0";
static TEST_STR:  &'static str = "¡Hola, amigos!";

fn owned_g_str(source: &str) -> OwnedGStr {
    assert!(source.ends_with("\0"));
    unsafe {
        let p = source.as_ptr();
        OwnedGStr::from_ptr(g_strdup(p as *const gchar))
    }
}

fn owned_g_str_from_bytes(source: &[u8]) -> OwnedGStr {
    assert!(source.last() == Some(&0u8));
    unsafe {
        let p = source.as_ptr();
        OwnedGStr::from_ptr(g_strdup(p as *const gchar))
    }
}

fn g_str_equal(p1: *const gchar, p2: *const gchar) -> bool {
    let cmp_res = unsafe { libc::strcmp(p1, p2) };
    cmp_res == 0
}

#[test]
fn test_owned_g_str_to_bytes() {
    let str = owned_g_str_from_bytes(b"a\x80\0");
    let bytes = str.to_bytes();
    assert_eq!(bytes.len(), 2);
    assert_eq!(bytes[0], b'a');
    assert_eq!(bytes[1], b'\x80');
}

#[test]
fn test_owned_g_str_clone() {
    let str1 = owned_g_str(TEST_CSTR);
    let str2 = str1.clone();
    let s = str::from_utf8(str2.to_bytes()).unwrap();
    assert_eq!(s, TEST_STR.to_string());
}

#[test]
fn test_owned_g_str_eq() {
    let s1 = owned_g_str(TEST_CSTR);
    let s2 = owned_g_str(TEST_CSTR);
    assert!(s1 == s2);
}

#[test]
fn test_owned_g_str_ne() {
    let s1 = owned_g_str(TEST_CSTR);
    let s2 = owned_g_str("This is not the string you are looking for\0");
    assert!(s1 != s2);
}

#[test]
fn test_owned_g_str_deref() {
    let s = owned_g_str(TEST_CSTR);
    let p = s.as_ptr();
    assert!(g_str_equal(p, TEST_CSTR.as_ptr() as *const gchar));
}

#[test]
fn test_g_str_macro() {
    let s = g_str!("Hello!");
    assert!(g_str_equal(s.as_ptr(), "Hello!\0".as_ptr() as *const gchar));
}

#[test]
fn test_g_utf8_macro() {
    let s = g_utf8!("Hello!");
    assert!(g_str_equal(s.as_ptr(), "Hello!\0".as_ptr() as *const gchar));
}

#[test]
fn test_utf8_from_static_str() {
    let s = gstr::Utf8::from_static_str(TEST_CSTR);
    assert_eq!(s.as_ptr(), TEST_CSTR.as_ptr() as *const gchar);
}

#[test]
fn test_utf8_string_from_str() {
    let s = TEST_STR.to_string();
    let buf = Utf8String::new(&s[..]).unwrap();
    assert!(g_str_equal(buf.as_ptr(), TEST_CSTR.as_ptr() as *const gchar));

    let res = Utf8String::new("got\0nul");
    let err = res.err().unwrap();
    assert_eq!(err.nul_position(), 3);
}

#[test]
fn test_utf8_string_from_string() {
    let s = TEST_STR.to_string();
    let buf = Utf8String::new(s).unwrap();
    assert!(g_str_equal(buf.as_ptr(), TEST_CSTR.as_ptr() as *const gchar));

    let s = "got\0nul".to_string();
    let res = Utf8String::new(s);
    let err = res.err().unwrap();
    assert_eq!(err.nul_position(), 3);
}
