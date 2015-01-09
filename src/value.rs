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

use ffi;
use gstr;
use gstr::{GStr, GStrArg};
use gtype::GType;
use object;
use object::ObjectType;
use types::{gboolean, gchar, gdouble, gfloat, gint, glong, gpointer};
use types::{guchar, guint, gulong};
use util::{escape_bytestring, is_true};

use std::mem;

pub type Value = ffi::GValue;

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { ffi::g_value_unset(self) };
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        unsafe {
            let mut val: Value = mem::zeroed();
            ffi::g_value_init(&mut val, self.g_type);
            ffi::g_value_copy(self, &mut val);
            val
        }
    }
}

impl Value {
    pub fn new(g_type: GType) -> Value {
        unsafe {
            let mut val: Value = mem::zeroed();
            ffi::g_value_init(&mut val, g_type.to_raw());
            val
        }
    }

    #[inline]
    pub fn value_type(&self) -> GType {
        unsafe { GType::new(self.g_type) }
    }

    pub fn get_boolean(&self) -> bool {
        is_true(unsafe { ffi::g_value_get_boolean(self) })
    }

    pub fn set_boolean(&mut self, val: bool) {
        unsafe { ffi::g_value_set_boolean(self, val as gboolean) };
    }

    pub fn get_char(&self) -> gchar {
        unsafe { ffi::g_value_get_char(self) }
    }

    pub fn set_char(&mut self, val: gchar) {
        unsafe { ffi::g_value_set_char(self, val) };
    }

    pub fn get_schar(&self) -> i8 {
        unsafe { ffi::g_value_get_schar(self) }
    }

    pub fn set_schar(&mut self, val: i8) {
        unsafe { ffi::g_value_set_schar(self, val) };
    }

    pub fn get_uchar(&self) -> guchar {
        unsafe { ffi::g_value_get_uchar(self) }
    }

    pub fn set_uchar(&mut self, val: guchar) {
        unsafe { ffi::g_value_set_uchar(self, val) };
    }

    pub fn get_int(&self) -> gint {
        unsafe { ffi::g_value_get_int(self) }
    }

    pub fn set_int(&mut self, val: gint) {
        unsafe { ffi::g_value_set_int(self, val) };
    }

    pub fn get_uint(&self) -> guint {
        unsafe { ffi::g_value_get_uint(self) }
    }

    pub fn set_uint(&mut self, val: guint) {
        unsafe { ffi::g_value_set_uint(self, val) };
    }

    pub fn get_long(&self) -> glong {
        unsafe { ffi::g_value_get_long(self) }
    }

    pub fn set_long(&mut self, val: glong) {
        unsafe { ffi::g_value_set_long(self, val) };
    }

    pub fn get_ulong(&self) -> gulong {
        unsafe { ffi::g_value_get_ulong(self) }
    }

    pub fn set_ulong(&mut self, val: gulong) {
        unsafe { ffi::g_value_set_ulong(self, val) };
    }

    pub fn get_int64(&self) -> i64 {
        unsafe { ffi::g_value_get_int64(self) }
    }

    pub fn set_int64(&mut self, val: i64) {
        unsafe { ffi::g_value_set_int64(self, val) };
    }

    pub fn get_uint64(&self) -> u64 {
        unsafe { ffi::g_value_get_uint64(self) }
    }

    pub fn set_uint64(&mut self, val: u64) {
        unsafe { ffi::g_value_set_uint64(self, val) };
    }

    pub fn get_float(&self) -> gfloat {
        unsafe { ffi::g_value_get_float(self) }
    }

    pub fn set_float(&mut self, val: gfloat) {
        unsafe { ffi::g_value_set_float(self, val) };
    }

    pub fn get_double(&self) -> gdouble {
        unsafe { ffi::g_value_get_double(self) }
    }

    pub fn set_double(&mut self, val: gdouble) {
        unsafe { ffi::g_value_set_double(self, val) };
    }

    pub fn get_string(&self) -> Option<&[u8]> {
        unsafe {
            let s = ffi::g_value_get_string(self);
            if s.is_null() {
                return None;
            }
            Some(gstr::parse_as_bytes(s, self))
        }
    }

    pub fn set_string(&mut self, val: &GStrArg) {
        unsafe { ffi::g_value_set_string(self, val.as_ptr()) }
    }

    pub fn set_static_string(&mut self, val: &str) {
        assert!(val.ends_with("\0"),
                "static string is not null-terminated: \"{}\"", val);
        let p = val.as_ptr() as *const gchar;
        unsafe { ffi::g_value_set_static_string(self, p) }
    }

    pub fn set_static_bytes(&mut self, val: &[u8]) {
        assert!(val.last() == Some(&0u8),
                "static byte string is not null-terminated: \"{}\"",
                escape_bytestring(val));
        let p = val.as_ptr() as *const gchar;
        unsafe { ffi::g_value_set_static_string(self, p) }
    }

    pub fn take_string(&mut self, consumed: GStr) {
        unsafe { ffi::g_value_take_string(self, consumed.into_inner()) }
    }

    pub fn get_object<T>(&self) -> Option<&T>
        where T: ObjectType
    {
        assert!(self.value_type() == object::type_of::<T>(),
                "GValue does not have the object type");  // FIXME: format the expected type
        let p = unsafe { ffi::g_value_get_object(self) as *const T };
        if p.is_null() {
            return None;
        }
        Some(unsafe { mem::copy_lifetime(self, &*p) })
    }

    pub fn set_object<T>(&mut self, val: &T)
        where T: ObjectType
    {
        let p = val as *const T;
        unsafe { ffi::g_value_set_object(self, p as gpointer) }
    }
}
