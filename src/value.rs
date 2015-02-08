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

use boxed;
use boxed::BoxedType;
use gstr::{GStr, OwnedGStr};
use gtype::GType;
use object;
use object::ObjectType;
use types::{gboolean, gchar, gdouble, gfloat, gint, glong, gpointer};
use types::{guchar, guint, gulong};
use util::is_true;

use gobject as ffi;

use std::mem;
use std::ops::Deref;

pub struct Value(ffi::GValue);

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { ffi::g_value_unset(self.as_mut_raw()) };
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        Value(unsafe {
            let mut raw: ffi::GValue = mem::zeroed();
            ffi::g_value_init(&mut raw, self.as_raw().g_type);
            ffi::g_value_copy(self.as_raw(), &mut raw);
            raw
        })
    }
}

impl Value {
    pub fn new(g_type: GType) -> Value {
        Value(unsafe {
            let mut raw: ffi::GValue = mem::zeroed();
            ffi::g_value_init(&mut raw, g_type.to_raw());
            raw
        })
    }

    #[inline]
    fn as_raw(&self) -> &ffi::GValue {
        &self.0
    }

    #[inline]
    fn as_mut_raw(&mut self) -> &mut ffi::GValue {
        &mut self.0
    }

    #[inline]
    pub fn value_type(&self) -> GType {
        unsafe { GType::from_raw(self.as_raw().g_type) }
    }

    pub fn get_boolean(&self) -> bool {
        is_true(unsafe { ffi::g_value_get_boolean(self.as_raw()) })
    }

    pub fn set_boolean(&mut self, val: bool) {
        unsafe { ffi::g_value_set_boolean(self.as_mut_raw(), val as gboolean) };
    }

    pub fn get_char(&self) -> gchar {
        unsafe { ffi::g_value_get_char(self.as_raw()) }
    }

    pub fn set_char(&mut self, val: gchar) {
        unsafe { ffi::g_value_set_char(self.as_mut_raw(), val) };
    }

    pub fn get_schar(&self) -> i8 {
        unsafe { ffi::g_value_get_schar(self.as_raw()) }
    }

    pub fn set_schar(&mut self, val: i8) {
        unsafe { ffi::g_value_set_schar(self.as_mut_raw(), val) };
    }

    pub fn get_uchar(&self) -> guchar {
        unsafe { ffi::g_value_get_uchar(self.as_raw()) }
    }

    pub fn set_uchar(&mut self, val: guchar) {
        unsafe { ffi::g_value_set_uchar(self.as_mut_raw(), val) };
    }

    pub fn get_int(&self) -> gint {
        unsafe { ffi::g_value_get_int(self.as_raw()) }
    }

    pub fn set_int(&mut self, val: gint) {
        unsafe { ffi::g_value_set_int(self.as_mut_raw(), val) };
    }

    pub fn get_uint(&self) -> guint {
        unsafe { ffi::g_value_get_uint(self.as_raw()) }
    }

    pub fn set_uint(&mut self, val: guint) {
        unsafe { ffi::g_value_set_uint(self.as_mut_raw(), val) };
    }

    pub fn get_long(&self) -> glong {
        unsafe { ffi::g_value_get_long(self.as_raw()) }
    }

    pub fn set_long(&mut self, val: glong) {
        unsafe { ffi::g_value_set_long(self.as_mut_raw(), val) };
    }

    pub fn get_ulong(&self) -> gulong {
        unsafe { ffi::g_value_get_ulong(self.as_raw()) }
    }

    pub fn set_ulong(&mut self, val: gulong) {
        unsafe { ffi::g_value_set_ulong(self.as_mut_raw(), val) };
    }

    pub fn get_int64(&self) -> i64 {
        unsafe { ffi::g_value_get_int64(self.as_raw()) }
    }

    pub fn set_int64(&mut self, val: i64) {
        unsafe { ffi::g_value_set_int64(self.as_mut_raw(), val) };
    }

    pub fn get_uint64(&self) -> u64 {
        unsafe { ffi::g_value_get_uint64(self.as_raw()) }
    }

    pub fn set_uint64(&mut self, val: u64) {
        unsafe { ffi::g_value_set_uint64(self.as_mut_raw(), val) };
    }

    pub fn get_float(&self) -> gfloat {
        unsafe { ffi::g_value_get_float(self.as_raw()) }
    }

    pub fn set_float(&mut self, val: gfloat) {
        unsafe { ffi::g_value_set_float(self.as_mut_raw(), val) };
    }

    pub fn get_double(&self) -> gdouble {
        unsafe { ffi::g_value_get_double(self.as_raw()) }
    }

    pub fn set_double(&mut self, val: gdouble) {
        unsafe { ffi::g_value_set_double(self.as_mut_raw(), val) };
    }

    pub fn get_string(&self) -> Option<&GStr> {
        unsafe {
            let ptr = ffi::g_value_get_string(self.as_raw());
            if ptr.is_null() {
                return None;
            }
            Some(GStr::from_ptr(ptr))
        }
    }

    pub fn set_string(&mut self, val: &GStr) {
        unsafe { ffi::g_value_set_string(self.as_mut_raw(), val.as_ptr()) }
    }

    pub fn set_static_string(&mut self, val: &'static GStr) {
        unsafe {
            ffi::g_value_set_static_string(self.as_mut_raw(), val.as_ptr())
        }
    }

    pub fn take_string(&mut self, consumed: OwnedGStr) {
        unsafe {
            let ptr = consumed.as_ptr() as *mut gchar;
            ffi::g_value_take_string(self.as_mut_raw(), ptr);
            mem::forget(consumed);
        }
    }

    pub fn get_object<T>(&self) -> Option<&T>
        where T: ObjectType
    {
        assert!(self.value_type() == object::type_of::<T>(),
                "GValue does not have the object type {}",
                object::type_of::<T>().name());
        let p = unsafe { ffi::g_value_get_object(self.as_raw()) as *const T };
        if p.is_null() {
            return None;
        }
        Some(unsafe { mem::copy_lifetime(self, &*p) })
    }

    pub fn set_object<T>(&mut self, val: &T)
        where T: ObjectType
    {
        let p = val as *const T;
        unsafe { ffi::g_value_set_object(self.as_mut_raw(), p as gpointer) }
    }

    fn assert_boxed_type<T>(&self)
        where T: BoxedType
    {
        assert!(self.value_type() == boxed::type_of::<T>(),
                "GValue does not have the boxed type {}",
                boxed::type_of::<T>().name());
    }

    pub fn dup_boxed<T>(&self) -> Option<T>
        where T: BoxedType
    {
        self.assert_boxed_type::<T>();
        let p = unsafe { ffi::g_value_dup_boxed(self.as_raw()) };
        if p.is_null() {
            return None;
        }
        Some(unsafe {
            let boxed: T = BoxedType::from_ptr(p);
            boxed
        })
    }

    pub fn deref_boxed<T>(&self) -> Option<&<T as Deref>::Target>
        where T: BoxedType + Deref
    {
        self.assert_boxed_type::<T>();
        let p = unsafe { ffi::g_value_get_boxed(self.as_raw()) };
        if p.is_null() {
            return None;
        }
        unsafe {
            let boxed: T = BoxedType::from_ptr(p);
            let ret = Some(mem::copy_lifetime(self, &*boxed));

            // Lose the box by "subliming" it back into the raw pointer
            let q = boxed.into_ptr();
            debug_assert!(q == p,
                "BoxedType invariant violation: into_ptr does not reverse from_ptr");

            ret
        }
    }

    pub fn take_boxed<T>(&mut self, val: T) where T: BoxedType {
        unsafe {
            let p = val.into_ptr();
            ffi::g_value_take_boxed(self.as_mut_raw(), p);
        }
    }
}
