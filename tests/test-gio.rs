// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use gio::File;
use gio::enums::io_error;
use grust::refcount::{Ref,SyncRef};
use grust::native::{LoopRunner,MainLoop};
use grust::object;
use grust::error;

fn run_on_mainloop(setup: |mainloop: &mut MainLoop|) {
    let runner = LoopRunner::new();
    runner.run_after(setup);
}

#[test]
fn new_ref() {
    let mut f = File::new_for_path("/dev/null");
    let mut g = Ref::new(f.borrow_mut());
    let path = g.borrow_mut().get_path().into_collection();
    assert_eq!(path.as_slice(), "/dev/null");
}

#[test]
fn clone() {
    let rf = File::new_for_path("/dev/null");
    let mut rg = rf.clone();
    let g = rg.borrow_mut();
    let path = g.get_path().into_collection();
    assert_eq!(path.as_slice(), "/dev/null");
}

#[test]
fn async() {
    run_on_mainloop(|mainloop| {
        let mut f = File::new_for_path("/dev/null");
        let mut rml = SyncRef::new(mainloop);
        f.read_async(0, None,
            box proc(obj, res) {
                let f: &mut File = object::cast_mut(obj);
                match f.read_finish(res) {
                    Ok(_)  => {}
                    Err(e) => { println!("Error: {}", e.message()) }
                }
                rml.quit();
            });
    })
}

#[test]
fn error_matches() {
    run_on_mainloop(|mainloop| {
        let mut rml = SyncRef::new(mainloop);
        let mut f = File::new_for_path("./does-not-exist");
        f.read_async(0, None,
            box proc(obj, res) {
                let f: &mut File = object::cast_mut(obj);
                match f.read_finish(res) {
                    Ok(_)  => { unreachable!() }
                    Err(e) => {
                        assert!(e.matches(io_error::NotFound));
                    }
                }
                rml.quit();
            });
    })
}

#[test]
fn error_to_domain() {
    run_on_mainloop(|mainloop| {
        let mut rml = SyncRef::new(mainloop);
        let mut f = File::new_for_path("./does-not-exist");
        f.read_async(0, None,
            box proc(obj, res) {
                let f: &mut File = object::cast_mut(obj);
                match f.read_finish(res) {
                    Ok(_)  => { unreachable!() }
                    Err(e) => {
                        match e.to_domain::<io_error::IOErrorEnum>() {
                            error::NotInDomain => { unreachable!() }
                            error::Unknown(code) => {
                                fail!("unknown error code {}", code)
                            }
                            error::Known(code) => {
                                assert_eq!(code, io_error::NotFound);
                            }
                        }
                    }
                }
                rml.quit();
            });
    })
}
