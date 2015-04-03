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

#![allow(trivial_numeric_casts)]
#![allow(unstable_features)]

#![feature(core)]

extern crate grust;

use grust::enumeration;
use grust::enumeration::{IntrospectedEnum, UnknownValue};

use grust::types::gint;
use std::num::from_i32;

#[derive(Copy, Debug, Eq, PartialEq, FromPrimitive)]
enum MyEnum {
    Foo = 1,
    Bar = 2,
}

impl IntrospectedEnum for MyEnum {

    fn from_int(v: gint) -> Result<Self, UnknownValue> {
        from_i32(v as i32).ok_or(UnknownValue(v))
    }

    fn to_int(&self) -> gint {
        *self as gint
    }

    fn name(&self) -> &'static str {
        match *self {
            MyEnum::Foo => "foo",
            MyEnum::Bar => "bar"
        }
    }
}

#[test]
fn test_enum_from_int() {
    let v: MyEnum = enumeration::from_int(1).unwrap();
    assert_eq!(v, MyEnum::Foo);
    let v: MyEnum = enumeration::from_int(2).unwrap();
    assert_eq!(v, MyEnum::Bar);
}

#[test]
fn test_unknown_value() {
    let res = enumeration::from_int::<MyEnum>(0);
    assert_eq!(res.err().unwrap(), UnknownValue(0))
}
