// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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
use ffi;
use ffi::GTypeInstance;
use refcount::RefcountFuncs;
use util::is_false;

use std::c_str::CString;
use std::mem::transmute;

pub trait ObjectType {
    fn get_type(&self) -> GType;
}

pub static REFCOUNT_FUNCS: RefcountFuncs = (
        &ffi::g_object_ref,
        &ffi::g_object_unref
    );

pub fn cast<'a, T: ObjectType, U: ObjectType>(source: &'a T)
                                             -> &'a U {
    unsafe {
        let inst = source as *const T as *const GTypeInstance;
        let dest: &'a U = transmute(source);
        let dest_type = dest.get_type();
        if is_false(ffi::g_type_check_instance_is_a(inst, dest_type)) {
            fail!("invalid cast to type {}",
                  CString::new(ffi::g_type_name(dest_type), false));
        }
        dest
    }
}

pub fn cast_mut<'a, T: ObjectType, U: ObjectType>(source: &'a mut T)
                                                  -> &'a mut U {
    unsafe {
        let inst = source as *mut T as *const T as *const GTypeInstance;
        let dest: &'a mut U = transmute(source);
        let dest_type = dest.get_type();
        if is_false(ffi::g_type_check_instance_is_a(inst, dest_type)) {
            fail!("invalid cast to type {}",
                  CString::new(ffi::g_type_name(dest_type), false));
        }
        dest
    }
}
