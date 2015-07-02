// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

extern crate grust;

use grust::mainloop;
use grust::mainloop::{LoopRunner, Source, SourceCallback};
use grust::mainloop::CallbackResult::{Continue, Remove};

use std::sync::mpsc;
use std::thread;

#[test]
fn test_invoke_once() {
    let runner = LoopRunner::new();
    runner.run_after(|mainloop| {
        const THREAD_NAME: &'static str = "invoker";
        thread::Builder::new().name(THREAD_NAME.to_string()).spawn(move || {
            let mlc = mainloop.clone();
            let ctx = mainloop.get_context();
            ctx.invoke(SourceCallback::once(move || {
                assert!(thread::current().name() != Some(THREAD_NAME));
                mlc.quit();
            }));
        }).unwrap();
    });
}

#[test]
fn test_invoke() {
    let runner = LoopRunner::new();
    runner.run_after(|mainloop| {
        const THREAD_NAME: &'static str = "invoker";
        thread::Builder::new().name(THREAD_NAME.to_string()).spawn(move || {
            let mlc = mainloop.clone();
            let mut count = 0;
            let ctx = mainloop.get_context();
            ctx.invoke(SourceCallback::new(move || {
                assert!(thread::current().name() != Some(THREAD_NAME));
                count += 1;
                if count < 2 {
                    Continue
                } else {
                    mlc.quit();
                    Remove
                }
            }));
        }).unwrap();
    });
}

#[test]
fn test_idle_source() {
    let runner = LoopRunner::new();
    runner.run_after(|ml| {
        let source = mainloop::idle_source_new();
        let mlc = ml.clone();
        let mut count = 0;
        source.set_callback(SourceCallback::new(move || {
            assert!(count <= 2);
            count += 1;
            if count < 2 {
                Continue
            } else {
                mlc.quit();
                Remove
            }
        }));
        source.attach(ml.get_context());
    });
}

#[test]
fn test_one_time_callback() {
    let runner = LoopRunner::new();
    runner.run_after(|ml| {
        let source = mainloop::idle_source_new();
        let mlc = ml.clone();
        source.set_callback(SourceCallback::once(move || {
            mlc.quit();
        }));
        source.attach(ml.get_context());
    });
}

#[test]
fn test_timeout_source() {
    let runner = LoopRunner::new();
    runner.run_after(|ml| {
        let source = mainloop::timeout_source_new(10);
        let mlc = ml.clone();
        source.set_callback(SourceCallback::once(move || {
            mlc.quit();
        }));
        source.attach(ml.get_context());
    });
}

#[test]
fn test_priority() {
    let (tx, rx) = mpsc::channel();
    let runner = LoopRunner::new();
    runner.run_after(|ml| {
        let source1 = mainloop::idle_source_new();
        source1.set_priority(mainloop::PRIORITY_DEFAULT);
        let mut count = 0;
        source1.set_callback(SourceCallback::new(move || {
            tx.send(()).unwrap();
            count += 1;
            if count == 1 {
                Remove
            } else {
                Continue
            }
        }));
        let source2 = mainloop::idle_source_new();
        let mlc = ml.clone();
        source2.set_callback(SourceCallback::once(move || {
            mlc.quit();
        }));
        let ctx = ml.get_context();
        source1.attach(ctx);
        source2.attach(ctx);
    });
    assert_eq!(rx.iter().count(), 1);
}

#[test]
fn test_attached_source() {
    let (tx, rx) = mpsc::channel();
    let runner = LoopRunner::new();
    runner.run_after(|ml| {
        let ctx = ml.get_context();
        let source1 = mainloop::idle_source_new();
        let attached = source1.attach(ctx);
        let attached_source: &Source = attached.as_ref();
        attached_source.set_priority(mainloop::PRIORITY_DEFAULT);
        let atc = attached.clone();
        attached.as_source().set_callback(SourceCallback::new(move || {
            tx.send(()).unwrap();
            atc.destroy();
            Continue
        }));
        let mlc = ml.clone();
        let source2 = mainloop::idle_source_new();
        source2.set_callback(SourceCallback::once(move || {
            mlc.quit();
        }));
        source2.attach(ctx);
    });
    assert_eq!(rx.iter().count(), 1);
}
