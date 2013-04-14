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

extern mod grust (name="grust", vers="0.1");
extern mod gio (name="grust-Gio", vers="2.0");

// We have to do this because of an rpath problem with crates linking to
// foreign libraries
pub extern mod grustna {
}

#[test]
fn main() {
    grust::init();
    let f = &gio::File::new_for_path("/dev/null") as &gio::File;
    assert!(f.get_path().to_str() == ~"/dev/null");
}
