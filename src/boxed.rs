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

use gstr::GStrBuf;
use gtype::GType;
use types::gpointer;

use gobject as ffi;

use std::any::TypeId;
use std::boxed::into_raw as box_into_raw;
use std::cell::RefCell;
use std::collections::hash_map::{HashMap,Entry};
use std::intrinsics::{get_tydesc, type_id};
use std::mem;

pub trait BoxedType {
    fn get_type() -> GType;
    unsafe fn into_ptr(self) -> gpointer;
    unsafe fn from_ptr(ptr: gpointer) -> Self;
}

pub fn type_of<T>() -> GType where T: BoxedType
{
    <T as BoxedType>::get_type()
}

extern "C" fn box_copy<T>(raw: gpointer) -> gpointer
    where T: Clone
{
    let boxed: Box<T> = unsafe { Box::from_raw(raw as *mut T) };
    let copy: Box<T> = boxed.clone();
    unsafe {
        // Prevent the original value from being dropped
        box_into_raw(boxed);
        box_into_raw(copy) as gpointer
    }
}

extern "C" fn box_free<T>(raw: gpointer) {
    let boxed: Box<T> = unsafe { Box::from_raw(raw as *mut T) };
    mem::drop(boxed);
}

type TypeMap = HashMap<TypeId, GType>;

thread_local! {
    static BOX_TYPE_REGISTRY: RefCell<TypeMap> = RefCell::new(HashMap::new())
}

// Get the uniformly generated unique name for a static Rust type,
// suitable for GType registration
fn box_type_name<T>() -> Vec<u8> where T: 'static {
    let rust_name = unsafe {
        let tydesc = get_tydesc::<T>();
        (*tydesc).name
    };

    // Prefix and escape the Rust name to get the human-readable part of
    // the GType name.
    // GType names can contain alphanumerics or any of '_-+'.
    let mut name = String::from_str("Grust-Box-").into_bytes();
    name.reserve(rust_name.len() + 17);
    name.extend(rust_name.bytes().map(|c| {
        match c {
            b'A' ... b'Z' |
            b'a' ... b'z' |
            b'0' ... b'9' |
            b'_' | b'-' | b'+' => c,
            _ => b'-'
        }
    }));
    // To ensure uniqueness, append the type ID hash
    write!(&mut name, "-{:x}", unsafe { type_id::<T>() }).unwrap();

    name
}

fn register_box_type<T>() -> GType where T: Clone, T: 'static {
    let name = GStrBuf::from_vec(box_type_name::<T>()).unwrap();
    unsafe {
        // The type could have been registered by another thread
        let mut raw = ffi::g_type_from_name(name.as_ptr());
        if raw == 0 {
            raw = ffi::g_boxed_type_register_static(name.as_ptr(),
                                                    box_copy::<T>,
                                                    box_free::<T>);
        }
        GType::from_raw(raw)
    }
}

impl<T> BoxedType for Box<T> where T: Clone + Send {

    fn get_type() -> GType {
        let rust_type = TypeId::of::<T>();
        BOX_TYPE_REGISTRY.with(|cell| {
            let mut map = cell.borrow_mut();
            match map.entry(rust_type) {
                Entry::Occupied(existing) => { *existing.get() }
                Entry::Vacant(slot) => {
                    let gtype = register_box_type::<T>();
                    slot.insert(gtype);
                    gtype
                }
            }
        })
    }

    unsafe fn into_ptr(self) -> gpointer {
        box_into_raw(self) as gpointer
    }

    unsafe fn from_ptr(raw: gpointer) -> Box<T> {
        Box::from_raw(raw as *mut T)
    }
}
