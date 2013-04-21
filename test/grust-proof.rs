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

use core::result::{Result,Ok};
use grust::eventloop::EventLoop;

// We have to do this because of an rpath problem with crates linking to
// foreign libraries
extern mod grustna {
}

fn tcase(test: &fn()) {
    grust::init();
    test();
}

fn tcase_result(test: &fn() -> Result<(),()>) {
    do tcase {
        let result = test();
        assert!(result == Ok(()));
    }
}

#[test]
fn simple() {
    do tcase {
        let fobj = gio::File::new_for_path("/dev/null");
        let f = fobj.interface() as &gio::File;
        assert!(f.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn new_ref() {
    do tcase {
        let fobj = gio::File::new_for_path("/dev/null");
        let gobj = fobj.interface().new_ref();
        let g = gobj.interface() as &gio::File;
        assert!(g.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn clone() {
    do tcase {
        let fobj = gio::File::new_for_path("/dev/null");
        let gobj = fobj.clone();
        let g = gobj.interface() as &gio::File;
        assert!(g.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn off_stack() {
    do tcase_result {
        let f = ~gio::File::new_for_path("/dev/null");
        do task::try {
            let f = f.interface() as &gio::File;
            let p = f.get_path();
            assert!(p.to_str() == ~"/dev/null");
        }
    }
}

#[test]
fn async() {
    do tcase {
        let fobj = gio::File::new_for_path("/dev/null");
        let f = fobj.interface() as &gio::File;
        let el = EventLoop::new();
        let elo = ~el.clone();
        f.read_async(0, None,
                |obj, res| {
                    let f: &gio::File = obj.cast::<gio::raw::GFile>()
                                        as &gio::File;
                    let in = f.read_finish(res);
                    elo.quit();
                });
        el.run();
    }
}
