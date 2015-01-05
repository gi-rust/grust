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

#[macro_use]
extern crate grust;

use grust::boxed;
use grust::gtype;
use grust::value::Value;

#[test]
fn test_string() {
    let mut value = Value::new(gtype::STRING);
    value.set_string(g_str!("Hello"));
    let s = value.get_string().unwrap().to_bytes();
    assert_eq!(s, b"Hello");
}

#[test]
fn test_boxed() {
    let mut value = Value::new(boxed::type_of::<Box<String>>());
    {
        let os = value.deref_boxed::<Box<String>>();
        assert_eq!(os, None);
    }
    {
        let ob = value.dup_boxed::<Box<String>>();
        assert_eq!(ob, None);
    }
    value.take_boxed(Box::new("Hello!".to_string()));
    let value = value.clone();
    {
        let s = value.deref_boxed::<Box<String>>().unwrap();
        assert_eq!(&s[], "Hello!");
    }
    {
        let b = value.dup_boxed::<Box<String>>().unwrap();
        assert_eq!(*b, "Hello!");
    }
}
