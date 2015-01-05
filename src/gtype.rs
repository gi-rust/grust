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

use ffi;
use gstr;
use object::ObjectType;
use util::is_true;

use std::fmt;

pub mod raw {
    pub type GType = ::ffi::GType;
}

#[derive(Copy, Eq, PartialEq)]
pub struct GType(raw::GType);

impl GType {

    #[inline]
    pub unsafe fn new(type_id: raw::GType) -> GType {
        GType(type_id)
    }

    #[inline]
    pub fn to_raw(&self) -> raw::GType {
        let GType(type_id) = *self;
        type_id
    }
}

impl fmt::Show for GType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = unsafe { ffi::g_type_name(self.to_raw()) };
        match unsafe { gstr::parse_as_utf8(&name) } {
            Ok(s) => write!(f, "{}", s),
            Err(..) => Err(fmt::Error)
        }
    }
}

pub fn check_instance_is_a<T>(inst: &T, type_id: GType) -> bool
    where T: ObjectType
{
    let raw_inst = inst as *const T as *const ffi::GTypeInstance;
    let raw_type = type_id.to_raw();
    is_true(unsafe { ffi::g_type_check_instance_is_a(raw_inst, raw_type) })
}
