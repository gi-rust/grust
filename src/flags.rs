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
use types::guint;

use std::error::Error as ErrorTrait;
use std::fmt;

pub trait IntrospectedFlags : Sized {
    fn from_uint(v: guint) -> Result<Self, UnknownFlags>;
    fn to_uint(&self) -> guint;
}

pub trait FlagsType : IntrospectedFlags {
    fn get_type() -> GType;
}

#[derive(Copy, Clone)]
pub struct UnknownFlags {
    actual: guint,
    known_mask: guint
}

impl UnknownFlags {
    pub fn new(flags: guint, known_mask: guint) -> UnknownFlags {
        UnknownFlags { actual: flags, known_mask: known_mask }
    }

    pub fn actual(&self) -> guint { self.actual }
    pub fn unknown(&self) -> guint { self.actual & !self.known_mask }
    pub fn known(&self) -> guint { self.actual & self.known_mask }
}

pub mod prelude {
    pub use super::{IntrospectedFlags, FlagsType, UnknownFlags};
    pub use gtype::GType;
    pub use types::guint;
}

impl ErrorTrait for UnknownFlags {
    fn description(&self) -> &str {
        "unknown bit flags encountered"
    }
}

impl fmt::Display for UnknownFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported bit flag value 0b{:b} (unknown flags: 0b{:b})",
               self.actual(), self.unknown())
    }
}

impl fmt::Debug for UnknownFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnknownFlags {{ actual: 0b{:b}, known_mask: 0b{:b} }}",
               self.actual, self.known_mask)
    }
}

pub fn from_uint<F>(v: guint) -> Result<F, UnknownFlags>
    where F: IntrospectedFlags
{
    IntrospectedFlags::from_uint(v)
}

pub fn type_of<F>() -> GType where F: FlagsType {
    <F as FlagsType>::get_type()
}
