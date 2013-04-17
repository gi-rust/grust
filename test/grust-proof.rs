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

fn tcase(test: &fn()) {
    grust::init();
    test();
}

#[test]
fn simple() {
    do tcase {
        let f = &gio::File::new_for_path("/dev/null") as &gio::File;
        assert!(f.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn clone() {
    do tcase {
        let f = &gio::File::new_for_path("/dev/null").clone() as &gio::File;
        assert!(f.get_path().to_str() == ~"/dev/null");
    }
}

// Crashes due to https://github.com/mozilla/rust/issues/5882
#[test]
#[ignore]
fn off_stack() {
    do tcase {
        let f = ~gio::File::new_for_path("/dev/null") as ~gio::File;
        do spawn {
            let p = f.get_path();
            assert!(p.to_str() == ~"/dev/null");
        }
    }
}

#[test]
fn off_stack_borrow() {
    do tcase {
        let f = ~gio::File::new_for_path("/dev/null");
        do spawn {
            let g = &*f as &gio::File;
            let p = g.get_path();
            assert!(p.to_str() == ~"/dev/null");
        }
    }
}
