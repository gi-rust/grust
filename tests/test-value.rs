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

#[macro_use]
extern crate grust;

extern crate "gobject-2_0-sys" as gobject;

use grust::boxed;
use grust::gtype;
use grust::gtype::GType;
use grust::value::Value;

#[test]
fn test_string() {
    let mut value = Value::new(gtype::STRING);
    {
        let os = value.get_string();
        assert!(os.is_none());
    }
    value.set_string(g_str!("Hello"));
    {
        let s = value.get_string().unwrap().to_bytes();
        assert_eq!(s, b"Hello");
    }
}

#[test]
fn test_reset() {
    let mut value = Value::new(gtype::STRING);
    value.set_string(g_str!("Hello"));
    value.reset();
    {
        let os = value.get_string();
        assert!(os.is_none());
    }
}

#[derive(Clone)]
struct MyData(String);

unsafe impl boxed::BoxRegistered for MyData {
    fn box_type() -> GType {
        use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
        use std::sync::{Once, ONCE_INIT};

        static REGISTERED: AtomicUsize = ATOMIC_USIZE_INIT;
        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| {
            let gtype = boxed::register_box_type::<MyData>("GrustTestMyData");
            REGISTERED.store(gtype.to_raw() as usize, Ordering::Release);
        });

        let raw = REGISTERED.load(Ordering::Acquire) as gobject::GType;
        unsafe { GType::from_raw(raw) }
    }
}

#[test]
fn test_boxed() {
    let mut value = Value::new(boxed::type_of::<Box<MyData>>());
    {
        let os = value.deref_boxed::<Box<MyData>>();
        assert!(os.is_none());
    }
    {
        let ob = value.dup_boxed::<Box<MyData>>();
        assert!(ob.is_none());
    }
    value.take_boxed(Box::new(MyData("Hello!".to_string())));
    let value = value.clone();
    {
        let d = value.deref_boxed::<Box<MyData>>().unwrap();
        let MyData(ref s) = *d;
        assert_eq!(&s[], "Hello!");
    }
    {
        let b = value.dup_boxed::<Box<MyData>>().unwrap();
        let MyData(ref s) = *b;
        assert_eq!(&s[], "Hello!");
    }
}

#[test]
#[should_fail]
fn test_deref_boxed_panic() {
    let value = Value::new(gtype::INT);
    let _ = value.deref_boxed::<Box<MyData>>();
}

#[test]
#[should_fail]
fn test_dup_boxed_panic() {
    let value = Value::new(gtype::INT);
    let _ = value.dup_boxed::<Box<MyData>>();
}
