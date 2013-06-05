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

use gio::File;

use std::result::{Result,Ok};
use std::comm::{Port,stream};
use std::libc;
use std::task::*;
use std::util;
// use std::timer::recv_timeout;
// use std::uv_global_loop;
use grust::eventloop::EventLoop;

// Test timeout in milliseconds
static TEST_TIMEOUT: uint = 3000u;

fn spawn_with_future(func: ~fn()) -> Port<TaskResult> {
    let mut (port, _) = stream::<TaskResult>();
    let mut task = task();
    do task.future_result |p| { port = p; };
    task.spawn(func);
    port
}

fn tcase(test: ~fn()) {
    grust::init();

    let port = spawn_with_future(test);

    // recv_timeout is broken, see https://github.com/mozilla/rust/issues/6089
    match Some(port.recv()) /* recv_timeout(&uv_global_loop::get(), TEST_TIMEOUT, &port) */ {
        Some(Success) => {}
        Some(Failure) => {
            fail!(~"test failed");
        }
        None => {
            error!("test timed out");
            unsafe { libc::abort(); }
        }
    }
}

fn tcase_result(test: ~fn() -> Result<(),()>) {
    do tcase {
        let result = test();
        assert!(result == Ok(()));
    }
}

#[test]
fn simple() {
    do tcase {
        let fobj = gio::file_new_for_path("/dev/null");
        let f = fobj.interface();
        assert!(f.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn as_interface() {
    do tcase {
        do gio::file_new_for_path("/dev/null").as_interface |f| {
            assert!(f.get_path().to_str() == ~"/dev/null");
        };
    }
}

#[test]
fn new_ref() {
    do tcase {
        let fobj = gio::file_new_for_path("/dev/null");
        let gobj = fobj.interface().new_ref();
        let g = gobj.interface();
        assert!(g.get_path().to_str() == ~"/dev/null");
    }
}

#[test]
fn clone() {
    do tcase {
        let fobj = gio::file_new_for_path("/dev/null");
        do fobj.clone().as_interface |g| {
            assert!(g.get_path().to_str() == ~"/dev/null");
        };
    }
}

#[test]
fn off_task() {
    do tcase_result {
        let f = ~gio::file_new_for_path("/dev/null");
        do try {
            let f = f.interface();
            let p = f.get_path();
            assert!(p.to_str() == ~"/dev/null");
        }
    }
}

#[test]
fn off_task_as_interface() {
    do tcase_result {
        let fobj = ~gio::file_new_for_path("/dev/null");
        do try {
            do fobj.as_interface |f| {
                let p = f.get_path();
                assert!(p.to_str() == ~"/dev/null");
            };
        }
    }
}

#[test]
fn async() {
    do tcase {
        let fobj = gio::file_new_for_path("/dev/null");
        let f = fobj.interface();
        let el = EventLoop::new();
        let elo = ~el.clone();
        do f.read_async(0, None) |obj, res| {
            let f: &gio::interfaces::File = obj.cast();
            let in = f.read_finish(res);
            util::ignore(in);
            elo.quit();
        };
        unsafe {
            el.run();
        }
    }
}

#[test]
#[ignore]  // See https://github.com/mzabaluev/grust/issues/4
fn async_off_task() {
    do tcase {
        let fobj = ~gio::file_new_for_path("/dev/null");
        let el = EventLoop::new();
        let elo = ~el.clone();
        do spawn {
            let f = fobj.interface();
            let elo2 = ~elo.clone();
            do f.read_async(0, None) |obj, res| {
                let f: &gio::interfaces::File = obj.cast();
                f.read_finish(res);
                elo2.quit();
            };
        }
        unsafe {
            el.run();
        }
    }
}

#[test]
fn async_off_stack() {
    do tcase {
        let fobj = ~gio::file_new_for_path("/dev/null");
        let el = EventLoop::new();
        let elo = ~el.clone();
        do spawn_sched(SingleThreaded) {
            let f = fobj.interface();
            let elo2 = ~elo.clone();
            do f.read_async(0, None) |obj, res| {
                let f: &gio::interfaces::File = obj.cast();
                f.read_finish(res);
                elo2.quit();
            };
        };
        unsafe {
            el.run();
        }
    }
}
