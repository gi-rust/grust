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

#![crate_name = "grust-Gio-2_0"]

#![crate_type = "lib"]

extern crate grust;
extern crate "grust-GLib-2_0" as glib;
extern crate "grust-GObject-2_0" as gobject;
extern crate libc;

use grust::gtype::GType;
use grust::marker;
use grust::object;
use grust::refcount;
use grust::types;
use grust::utf8;

use cast::AsyncResult as _cast_AsyncResult;
use cast::Cancellable as _cast_Cancellable;

#[repr(C)]
pub struct AsyncResult {
    marker: marker::ObjectMarker
}

#[repr(C)]
pub struct File {
    marker: marker::ObjectMarker
}

#[repr(C)]
pub struct Cancellable {
    parent_instance: gobject::Object,
    _priv: types::gpointer
}

#[repr(C)]
pub struct InputStream {
    parent_instance: gobject::Object,
    _priv: types::gpointer
}

#[repr(C)]
pub struct FileInputStream {
    parent_instance: InputStream,
    _priv: types::gpointer
}

pub mod enums {

    pub mod io_error {

        use grust::error;
        use grust::quark;
        use std::fmt;
        use std::sync::atomic;

        #[deriving(FromPrimitive,PartialEq,Eq)]
        #[repr(C)]
        pub enum IOErrorEnum {
            Failed = 0,
            NotFound = 1,
            Exists = 2,
            // ...
        }

        impl fmt::Show for IOErrorEnum {
            fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
                let s: &'static str = match *self {
                    IOErrorEnum::Failed   => "failed",
                    IOErrorEnum::NotFound => "not-found",
                    IOErrorEnum::Exists   => "exists",
                    // ...
                };
                s.fmt(format)
            }
        }

        static mut ERROR_QUARK: atomic::AtomicUint = atomic::INIT_ATOMIC_UINT;

        impl error::ErrorDomain for IOErrorEnum {
            fn error_domain(_: Option<IOErrorEnum>) -> quark::Quark {
                unsafe {
                    let mut d = ERROR_QUARK.load(atomic::Relaxed)
                                as quark::Quark;
                    if d == 0 {
                        d = quark::from_static_string("g-io-error-quark\0");
                        ERROR_QUARK.store(d as uint, atomic::Relaxed);
                    }
                    d
                }
            }
        }

        impl ToPrimitive for IOErrorEnum {
            fn to_int(&self) -> Option<int> { Some(*self as int) }
            fn to_i64(&self) -> Option<i64> { Some(*self as i64) }
            fn to_u64(&self) -> Option<u64> { Some(*self as u64) }
        }
    }
}

#[allow(improper_ctypes)]
pub mod raw {
    use grust::types::{gchar,gint,gpointer};
    use grust::gtype::GType;
    use grust::error::raw::GError;
    use gobject;
    use libc;

    pub type GAsyncResult = super::AsyncResult;
    pub type GFile = super::File;
    pub type GCancellable = super::Cancellable;
    pub type GInputStream = super::InputStream;
    pub type GFileInputStream = super::FileInputStream;

    pub type GAsyncReadyCallback = extern "C" fn(source_object: *mut gobject::raw::GObject,
                                                 res: *mut GAsyncResult,
                                                 user_data: gpointer);  

    #[link(name="gio-2.0")]
    extern {
        pub fn g_async_result_get_type() -> GType;
        pub fn g_file_get_type() -> GType;
        pub fn g_file_new_for_path(path: *const gchar) -> *mut GFile;
        pub fn g_file_get_path(file: *mut GFile) -> *mut libc::c_char;
        pub fn g_file_read_async(file: *mut GFile,
                                 io_priority: gint,
                                 cancellable: *mut GCancellable,
                                 callback: GAsyncReadyCallback,
                                 user_data: gpointer);
        pub fn g_file_read_finish(file: *mut GFile,
                                  res: *mut GAsyncResult,
                                  error: *mut *mut GError)
                                  -> *mut GFileInputStream;
    }
}

pub mod async {

    use gobject;

    pub type AsyncReadyCallback =
                for<'a> proc(&'a mut gobject::Object,
                             &'a mut super::AsyncResult)
                            : Send;
}

mod async_shim {

    use grust::types;
    use super::async;
    use super::raw;
    use gobject;
    use std::mem;

    pub extern "C" fn async_ready_callback(source_object: *mut gobject::raw::GObject,
                                           res: *mut raw::GAsyncResult,
                                           user_data: types::gpointer) {
        unsafe {
            let callback: Box<async::AsyncReadyCallback> =
                    mem::transmute(user_data);

            (*callback)(&mut *source_object, &mut *res);
        }
    }
}

pub mod cast {

    use gobject;

    pub trait AsyncResult {
        fn as_gio_async_result<'a>(&'a self) -> &'a super::AsyncResult;
        fn as_mut_gio_async_result<'a>(&'a mut self) -> &'a mut super::AsyncResult;
    }

    pub trait Cancellable : gobject::cast::Object {
        fn as_gio_cancellable<'a>(&'a self) -> &'a super::Cancellable;
        fn as_mut_gio_cancellable<'a>(&'a mut self) -> &'a mut super::Cancellable;
    }

    pub trait InputStream : gobject::cast::Object {
    }

    pub trait FileInputStream : InputStream {
    }

    pub trait File {
        fn as_gio_file<'a>(&'a self) -> &'a super::File;
        fn as_mut_gio_file<'a>(&'a mut self) -> &'a mut super::File;
    }
}

impl File {

    pub fn new_for_path<T: utf8::ToUTF8>(path: T) -> refcount::Ref<File> {
        let ret = path.with_utf8_c_str(|p| {
            unsafe {
                raw::g_file_new_for_path(p)
            }
        });
        unsafe { refcount::raw::ref_from_ptr(ret) }
    }

    pub fn get_path<'a>(&mut self) -> utf8::UTF8Buf {
        unsafe {
            let ret = raw::g_file_get_path(self);
            utf8::UTF8Buf::wrap(ret)
        }
    }

    pub fn read_async(&mut self,
                      io_priority: types::gint,
                      cancellable: Option<&mut Cancellable>,
                      callback: Box<async::AsyncReadyCallback>) {
        unsafe {
            let raw_cancellable =
                match cancellable {
                    Some(c) => c.as_mut_gio_cancellable() as *mut raw::GCancellable,
                    None    => std::ptr::null_mut::<raw::GCancellable>()
                };
            let raw_callback: types::gpointer = std::mem::transmute(callback);

            raw::g_file_read_async(self,
                                   io_priority as libc::c_int,
                                   raw_cancellable,
                                   async_shim::async_ready_callback,
                                   raw_callback);
        }
    }

    pub fn read_finish(&mut self, res: &mut AsyncResult)
                      -> std::result::Result<refcount::Ref<FileInputStream>,
                                             grust::error::Error> {
        unsafe {
            let mut err: grust::error::Error = grust::error::unset();
            let ret = raw::g_file_read_finish(self,
                                              res.as_mut_gio_async_result(),
                                              err.slot_ptr());
            if err.is_set() {
                std::result::Err(err)
            } else {
                std::result::Ok(refcount::raw::ref_from_ptr(ret))
            }
        }
    }
}

impl object::ObjectType for AsyncResult {
    fn get_type(&self) -> GType {
        unsafe {
            raw::g_async_result_get_type()
        }
    }
}

impl object::ObjectType for File {
    fn get_type(&self) -> GType {
        unsafe {
            raw::g_file_get_type()
        }
    }
}

impl refcount::Refcount for File {
    fn refcount_funcs(&self) -> &'static refcount::RefcountFuncs {
        &object::REFCOUNT_FUNCS
    }
}

impl refcount::Refcount for FileInputStream {
    fn refcount_funcs(&self) -> &'static refcount::RefcountFuncs {
        &object::REFCOUNT_FUNCS
    }
}

impl cast::AsyncResult for AsyncResult {
    fn as_gio_async_result<'a>(&'a self) -> &'a AsyncResult { self }
    fn as_mut_gio_async_result<'a>(&'a mut self) -> &'a mut AsyncResult { self }
}

impl gobject::cast::Object for Cancellable {
    fn as_gobject_object<'a>(&'a self) -> &'a gobject::Object {
        &self.parent_instance
    }
    fn as_mut_gobject_object<'a>(&'a mut self) -> &'a mut gobject::Object {
        &mut self.parent_instance
    }
}

impl cast::Cancellable for Cancellable {
    fn as_gio_cancellable<'a>(&'a self) -> &'a Cancellable { self }
    fn as_mut_gio_cancellable<'a>(&'a mut self) -> &'a mut Cancellable { self }
}

impl cast::File for File {
    fn as_gio_file<'a>(&'a self) -> &'a File { self }
    fn as_mut_gio_file<'a>(&'a mut self) -> &'a mut File { self }
}

impl gobject::cast::Object for InputStream {
    fn as_gobject_object<'a>(&'a self) -> &'a gobject::Object {
        &self.parent_instance
    }
    fn as_mut_gobject_object<'a>(&'a mut self) -> &'a mut gobject::Object {
        &mut self.parent_instance
    }
}

impl gobject::cast::Object for FileInputStream {
    fn as_gobject_object<'a>(&'a self) -> &'a gobject::Object {
        self.parent_instance.as_gobject_object()
    }
    fn as_mut_gobject_object<'a>(&'a mut self) -> &'a mut gobject::Object {
        self.parent_instance.as_mut_gobject_object()
    }
}
