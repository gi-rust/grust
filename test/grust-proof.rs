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

extern crate grust;
extern crate "grust-Gio-2_0" as gio;

use gio::File;
use gio::interface::File as FileIface;
use grust::refcount::{Ref,SyncRef};
use grust::native::LoopRunner;
use grust::object;

// Test timeout in milliseconds
// static TEST_TIMEOUT: uint = 3000u;

fn tcase(test: proc(): Send) {
    test()
}

#[test]
fn simple() {
    tcase(proc() {
        let mut f = File::new_for_path("/dev/null");
        let path = f.borrow_mut().get_path();
        assert!(path.equiv(&"/dev/null"));
    })
}

#[test]
fn as_interface() {
    tcase(proc() {
        let mut r = File::new_for_path("/dev/null");
        let f = r.borrow_mut() as &mut FileIface;
        let path = f.get_path();
        assert_eq!(path.into_string(), String::from_str("/dev/null"));
    })
}

#[test]
fn new_ref() {
    tcase(proc() {
        let mut f = File::new_for_path("/dev/null");
        let mut g = Ref::new(f.borrow_mut());
        let path = g.borrow_mut().get_path();
        assert_eq!(path.into_string().as_slice(), "/dev/null");
    })
}

#[test]
fn clone() {
    tcase(proc() {
        let rf = File::new_for_path("/dev/null");
        let mut rg = rf.clone();
        let g = rg.borrow_mut();
        let path = g.get_path();
        assert_eq!(path.into_string().as_slice(), "/dev/null");
    })
}

#[test]
fn async() {
    tcase(proc() {
        let mut f = File::new_for_path("/dev/null");
        let runner = LoopRunner::new();
        runner.run_after(|mainloop| {
            let mut rml = SyncRef::new(mainloop);
            f.read_async(0, None,
                box proc(obj, res) {
                    let f: &mut File = object::cast_mut(obj);
                    match f.read_finish(res) {
                        Ok(_)  => {}
                        Err(_) => { println!("Error!") }  // TODO: impl Fmt for Error
                    }
                    rml.quit();
                });
        });
    })
}
