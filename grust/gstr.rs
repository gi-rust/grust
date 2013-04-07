/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301  USA
 */

use grust::types::*;
use glib::raw::symbols::{g_free, g_strdup};

pub struct GStr {
    priv data: *gchar,
}

impl GStr {
    pub unsafe fn wrap(data: *gchar) -> GStr { GStr{ data: data } }
}

impl Drop for GStr {
    fn finalize(&self) {
        unsafe {
            g_free(self.data as *());
        }
    }
}

impl Clone for GStr {
    fn clone(&self) -> GStr {
        unsafe {
            GStr::wrap(g_strdup(self.data))
        }
    }
}

impl ToStr for GStr {
    fn to_str(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(self.data)
        }
    }
}

impl Eq for GStr {
    fn eq(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }

    fn ne(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) != 0
        }
    }
}

impl TotalEq for GStr {
    fn equals(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }
}
