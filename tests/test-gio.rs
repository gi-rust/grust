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

use gio::{File, FileInputStream, IOErrorEnum};
use gio::cast::AsFile;
use grust::refcount::Ref;
use grust::mainloop::{LoopRunner,MainLoop};
use grust::object;
use grust::error::Match as ErrorMatch;
use std::error::Error;

fn run_on_mainloop<F>(setup: F) where F: FnOnce(Ref<MainLoop>) {
    let runner = LoopRunner::new();
    runner.run_after(setup);
}

#[test]
fn as_file() {
    let f = File::new_for_path(g_utf8!("/dev/null"));
    let g = f.as_file();
    let path = g.get_path();
    assert_eq!(path.parse_as_utf8().unwrap(), "/dev/null");
}

#[test]
fn deref() {
    let f = File::new_for_path(g_utf8!("/dev/null"));
    let path = f.get_path();
    assert_eq!(path.parse_as_utf8().unwrap(), "/dev/null");
}

#[test]
fn new_ref() {
    let f = File::new_for_path(g_utf8!("/dev/null"));
    let g = Ref::new(&*f);
    let path = g.get_path();
    assert_eq!(path.parse_as_utf8().unwrap(), "/dev/null");
}

#[test]
fn clone() {
    let rf = File::new_for_path(g_utf8!("/dev/null"));
    let rg = rf.clone();
    let path = rg.get_path();
    assert_eq!(path.parse_as_utf8().unwrap(), "/dev/null");
}

#[test]
#[should_fail]
fn cast_fail() {
    let rf = File::new_for_path(g_utf8!("/dev/null"));
    let _: &FileInputStream = object::cast(&*rf);
}

#[test]
fn async() {
    run_on_mainloop(|mainloop| {
        let f = File::new_for_path(g_utf8!("/dev/null"));
        f.read_async(0, None,
            move |obj, res| {
                let f: &File = object::cast(obj);
                match f.read_finish(res) {
                    Ok(_)  => {}
                    Err(e) => { println!("Error: {}", e.description()) }
                }
                mainloop.quit();
            });
    })
}

#[test]
fn error_to_domain() {
    run_on_mainloop(|mainloop| {
        let f = File::new_for_path(g_utf8!("./does-not-exist"));
        f.read_async(0, None,
            move |obj, res| {
                let f: &File = object::cast(obj);
                match f.read_finish(res) {
                    Ok(_)  => { unreachable!() }
                    Err(e) => {
                        match IOErrorEnum::from_error(&e) {
                            ErrorMatch::Known(code) => {
                                assert_eq!(code, IOErrorEnum::NotFound);
                            }
                            ErrorMatch::Unknown(code) => {
                                panic!("unknown error code {}", code)
                            }
                            _ => unreachable!()
                        }
                    }
                }
                mainloop.quit();
            });
    })
}

#[test]
fn error_match_partial_eq() {
    run_on_mainloop(|mainloop| {
        let f = File::new_for_path(g_utf8!("./does-not-exist"));
        f.read_async(0, None,
            move |obj, res| {
                let f: &File = object::cast(obj);
                match f.read_finish(res) {
                    Ok(_)  => { unreachable!() }
                    Err(e) => {
                        assert_eq!(IOErrorEnum::from_error(&e),
                                   ErrorMatch::Known(IOErrorEnum::NotFound));
                    }
                }
                mainloop.quit();
            });
    })
}
