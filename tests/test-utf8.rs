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

use grust::utf8::{UTF8Buf,UTF8Str,ToUTF8};

use grust::types::gchar;

use glib::raw::g_strdup;

use std::string;

static TEST_CSTR: &'static str = "¡Hola, amigos!\0";
static TEST_STR:  &'static str = "¡Hola, amigos!";

fn new_test_buf(source: &str) -> UTF8Buf {
    assert!(source.ends_with("\0"));
    unsafe {
        let p = source.as_ptr();
        UTF8Buf::wrap(g_strdup(p as *const gchar))
    }
}

fn new_test_str(source: &str) -> UTF8Str {
    new_test_buf(source).into_collection()
}

#[test]
fn buf_to_string() {
    let buf = new_test_buf(TEST_CSTR);
    let s = buf.to_string();
    assert_eq!(s, String::from_str(TEST_STR));
}

#[test]
fn buf_clone() {
    let buf1 = new_test_buf(TEST_CSTR);
    let buf2 = buf1.clone();
    let s = buf2.to_string();
    assert_eq!(s, String::from_str(TEST_STR));
}

#[test]
fn buf_into_string() {
    let buf = new_test_buf(TEST_CSTR);
    let s = buf.into_string();
    assert_eq!(s, String::from_str(TEST_STR));
}

#[test]
fn buf_into_collection() {
    let buf = new_test_buf(TEST_CSTR);
    let s = buf.into_collection();
    assert_eq!(s.as_slice(), TEST_STR);
}

#[test]
fn buf_to_c_str() {
    let buf = new_test_buf(TEST_CSTR);
    let cs = buf.to_c_str();
    assert_eq!(cs, TEST_STR.to_c_str());
}

#[test]
fn buf_with_c_str() {
    let buf = new_test_buf(TEST_CSTR);
    buf.with_c_str(|p| {
        let s = unsafe { string::raw::from_buf(p as *const u8) };
        assert_eq!(s, String::from_str(TEST_STR));
    });
}

#[test]
fn buf_with_utf8_str() {
    let buf = new_test_buf(TEST_CSTR);
    buf.with_utf8_c_str(|p| {
        let s = unsafe { string::raw::from_buf(p as *const u8) };
        assert_eq!(s, String::from_str(TEST_STR));
    });
}

#[test]
fn str_clone() {
    let s1 = new_test_str(TEST_CSTR);
    let s2 = s1.clone();
    let s = s2.into_string();
    assert_eq!(s, String::from_str(TEST_STR));
}

#[test]
fn str_len() {
    let s = new_test_str(TEST_CSTR);
    assert_eq!(s.len(), TEST_STR.len());
}

#[test]
fn str_to_c_str() {
    let s = new_test_str(TEST_CSTR);
    let cs = s.to_c_str();
    assert_eq!(cs, TEST_STR.to_c_str());
}

#[test]
fn str_with_c_str() {
    let s = new_test_str(TEST_CSTR);
    s.with_c_str(|p| {
        let s = unsafe { string::raw::from_buf(p as *const u8) };
        assert_eq!(s, String::from_str(TEST_STR));
    });
}

#[test]
fn str_with_utf8_str() {
    let s = new_test_str(TEST_CSTR);
    s.with_utf8_c_str(|p| {
        let s = unsafe { string::raw::from_buf(p as *const u8) };
        assert_eq!(s, String::from_str(TEST_STR));
    });
}

#[test]
fn str_eq() {
    let s1 = new_test_str(TEST_CSTR);
    let s2 = new_test_str(TEST_CSTR);
    assert!(s1 == s2);
}

#[test]
fn str_ne() {
    let s1 = new_test_str(TEST_CSTR);
    let s2 = new_test_str("This is not the string you are looking for\0");
    assert!(s1 != s2);
}

#[test]
fn chars() {
    let s = new_test_str("A\u0410\u4E00\U00100000.\0");
    let mut chars = s.chars();
    assert_eq!(chars.next().unwrap(), 'A');
    assert_eq!(chars.next().unwrap(), '\u0410');
    assert_eq!(chars.next().unwrap(), '\u4E00');
    assert_eq!(chars.next().unwrap(), '\U00100000');
    assert_eq!(chars.next().unwrap(), '.');
    assert_eq!(chars.next(), None);
}
