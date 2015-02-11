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

use gtype::GType;
use types::gint;

use std::error::Error as ErrorTrait;
use std::fmt;

pub trait IntrospectedEnum {
    fn from_int(v: gint) -> Result<Self, UnknownValue>;
    fn to_int(&self) -> gint;
    fn name(&self) -> &'static str;
}

pub trait EnumType : IntrospectedEnum {
    fn get_type() -> GType;
}

#[derive(Copy, Debug, Eq, PartialEq)]
pub struct UnknownValue(pub gint);

impl ErrorTrait for UnknownValue {
    fn description(&self) -> &str {
        "unknown enumeration value"
    }
}

impl fmt::Display for UnknownValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown enumeration value {}", self.0)
    }
}

pub fn from_int<E>(v: gint) -> Result<E, UnknownValue>
    where E: IntrospectedEnum
{
    IntrospectedEnum::from_int(v)
}

pub fn type_of<E>() -> GType where E: EnumType {
    <E as EnumType>::get_type()
}
