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

use gstr::GStr;
use types::gchar;
use util::escape_bytestring;

use glib as ffi;

use std::fmt;
use std::sync::atomic;

#[derive(Copy, Eq, PartialEq)]
pub struct Quark(ffi::GQuark);

pub struct StaticQuark(pub &'static [u8], pub atomic::AtomicUsize);

impl Quark {

    #[inline]
    pub unsafe fn from_raw(raw: ffi::GQuark) -> Quark {
        Quark(raw)
    }

    pub fn from_static_str(s: &'static str) -> Quark {
        if !s.ends_with("\0") {
            panic!("static string is not null-terminated: \"{}\"", s);
        }
        unsafe { Quark::from_static_internal(s.as_bytes()) }
    }

    pub fn from_static_bytes(bytes: &'static [u8]) -> Quark {
        assert!(!bytes.is_empty());
        if bytes[bytes.len() - 1] != 0 {
            panic!("static byte string is not null-terminated: \"{}\"",
                   escape_bytestring(bytes));
        }
        unsafe { Quark::from_static_internal(bytes) }
    }

    unsafe fn from_static_internal(s: &'static [u8]) -> Quark {
        let p = s.as_ptr() as *const gchar;
        let q = ffi::g_quark_from_static_string(p);
        Quark(q)
    }

    pub fn to_g_str(&self) -> &'static GStr {
        let Quark(raw) = *self;
        unsafe {
            let s = ffi::g_quark_to_string(raw);
            GStr::from_ptr(s)
        }
    }

    #[inline]
    pub fn to_bytes(&self) -> &'static [u8] {
        self.to_g_str().to_bytes()
    }

    #[inline]
    pub fn to_raw(&self) -> ffi::GQuark {
        self.0
    }
}

impl fmt::Display for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.to_bytes()))
    }
}

impl fmt::Debug for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", escape_bytestring(self.to_bytes()))
    }
}

impl StaticQuark {

    pub fn get(&self) -> Quark {
        let StaticQuark(s, ref cached) = *self;
        let q = cached.load(atomic::Ordering::Relaxed) as ffi::GQuark;
        if q != 0 {
            Quark(q)
        } else {
            let quark = Quark::from_static_bytes(s);
            cached.store(quark.to_raw() as usize, atomic::Ordering::Relaxed);
            quark
        }
    }
}
