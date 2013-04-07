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

use grust;
use grust::plumbing;

pub mod raw {
    use grust::types::*;

    pub trait GFile { }

    #[link_name="gio-2.0"]
    pub extern mod symbols {
        fn g_file_new_for_path(path: *gchar) -> *GFile;
        fn g_file_get_path(file: *GFile) -> *gchar;
    }
}

pub trait File {
    fn get_path(&self) -> grust::GStr;
}

impl File {
    pub fn new_for_path(path: &str) -> plumbing::Object<raw::GFile> {
        unsafe {
            plumbing::wrap_object(str::as_c_str(path,
                    raw::symbols::g_file_new_for_path))
        }
    }
}

impl File for plumbing::Object<raw::GFile> {
    fn get_path(&self) -> grust::GStr {
        unsafe {
            let ret = raw::symbols::g_file_get_path(self.unwrap());
            grust::GStr::wrap(ret)
        }
    }
}
