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

#![feature(unboxed_closures)]

extern crate grust;
extern crate "grust-GLib-2_0" as glib;
extern crate "grust-GObject-2_0" as gobject;
extern crate libc;

use grust::callback;
use grust::error;
use grust::gstr;
use grust::gstr::IntoUtf8;
use grust::gtype::GType;
use grust::marker;
use grust::object;
use grust::quark;
use grust::refcount;
use grust::types;

use cast::AsAsyncResult;
use cast::AsCancellable;

use std::fmt;
use std::sync::atomic;

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

#[deriving(Copy, PartialEq, Eq, FromPrimitive)]
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

impl IOErrorEnum {

    pub fn error_domain() -> quark::Quark {
        static DOMAIN: quark::StaticQuark
            = quark::StaticQuark(b"g-io-error-quark\0",
                                 atomic::INIT_ATOMIC_UINT);
        DOMAIN.get()
    }

    pub fn from_error(err: &error::Error) -> error::Match<IOErrorEnum> {
        let (domain, code) = err.key();
        if domain != IOErrorEnum::error_domain() {
            return error::Match::NotInDomain;
        }
        if let Some(v) = FromPrimitive::from_int(code) {
            error::Match::Known(v)
        } else {
            error::Match::Unknown(code)
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
        pub fn g_file_input_stream_get_type() -> GType;
    }
}

mod async_shim {

    use grust::callback;
    use grust::types;
    use super::raw;
    use gobject;

    pub extern "C" fn async_ready_callback(source_object: *mut gobject::raw::GObject,
                                           res: *mut raw::GAsyncResult,
                                           user_data: types::gpointer) {
        unsafe {
            callback::invoke(user_data, (&mut *source_object, &mut *res))
        }
    }
}

pub mod cast {
    use gobject;

    pub trait AsAsyncResult {
        fn as_gio_async_result(&self) -> &super::AsyncResult;
        fn as_mut_gio_async_result(&mut self) -> &mut super::AsyncResult;
    }

    pub trait AsCancellable : gobject::cast::AsObject {
        fn as_gio_cancellable(&self) -> &super::Cancellable;
        fn as_mut_gio_cancellable(&mut self) -> &mut super::Cancellable;
    }

    pub trait AsInputStream : gobject::cast::AsObject {
        fn as_gio_input_stream(&self) -> &super::InputStream;
        fn as_mut_gio_input_stream(&mut self) -> &mut super::InputStream;
    }

    pub trait AsFileInputStream : AsInputStream {
        fn as_gio_file_input_stream(&self) -> &super::FileInputStream;
        fn as_mut_gio_file_input_stream(&mut self) -> &mut super::FileInputStream;
    }

    pub trait AsFile {
        fn as_gio_file(&self) -> &super::File;
        fn as_mut_gio_file(&mut self) -> &mut super::File;
    }
}

impl File {

    // TODO: need a macro for static UTF8In literals
    // to make the argument &UTF8In without having to put tedious code
    // into existing tests
    pub fn new_for_path(path: &str) -> refcount::Ref<File> {
        let p = path.into_utf8().unwrap();
        unsafe {
            let ret = raw::g_file_new_for_path(p.as_ptr());
            refcount::raw::ref_from_ptr(ret)
        }
    }

    pub fn get_path<'a>(&mut self) -> gstr::GStr {
        unsafe {
            let ret = raw::g_file_get_path(self);
            gstr::GStr::from_raw_buf(ret)
        }
    }

    pub fn read_async<F>(&mut self,
                         io_priority: types::gint,
                         cancellable: Option<&mut Cancellable>,
                         callback: F)
        where F : FnOnce(&mut gobject::Object, &mut AsyncResult) + Send
    {
        unsafe {
            let cancellable =
                match cancellable {
                    Some(c) => c.as_mut_gio_cancellable() as *mut raw::GCancellable,
                    None    => std::ptr::null_mut::<raw::GCancellable>()
                };
            let callback = callback::AsyncCallback::new(callback).into_raw_ptr();

            raw::g_file_read_async(self,
                                   io_priority as libc::c_int,
                                   cancellable,
                                   async_shim::async_ready_callback,
                                   callback);
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
                std::result::Result::Err(err)
            } else {
                std::result::Result::Ok(refcount::raw::ref_from_ptr(ret))
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

impl object::ObjectType for FileInputStream {
    fn get_type(&self) -> GType {
        unsafe {
            raw::g_file_input_stream_get_type()
        }
    }
}

impl cast::AsAsyncResult for AsyncResult {

    #[inline]
    fn as_gio_async_result(&self) -> &AsyncResult { self }

    #[inline]
    fn as_mut_gio_async_result(&mut self) -> &mut AsyncResult { self }
}

impl gobject::cast::AsObject for Cancellable {

    #[inline]
    fn as_gobject_object(&self) -> &gobject::Object {
        &self.parent_instance
    }

    #[inline]
    fn as_mut_gobject_object(&mut self) -> &mut gobject::Object {
        &mut self.parent_instance
    }
}

impl cast::AsCancellable for Cancellable {

    #[inline]
    fn as_gio_cancellable(&self) -> &Cancellable { self }

    #[inline]
    fn as_mut_gio_cancellable(&mut self) -> &mut Cancellable { self }
}

impl cast::AsFile for File {

    #[inline]
    fn as_gio_file(&self) -> &File { self }

    #[inline]
    fn as_mut_gio_file(&mut self) -> &mut File { self }
}

impl gobject::cast::AsObject for InputStream {

    #[inline]
    fn as_gobject_object(&self) -> &gobject::Object {
        &self.parent_instance
    }

    #[inline]
    fn as_mut_gobject_object<'a>(&mut self) -> &mut gobject::Object {
        &mut self.parent_instance
    }
}

impl gobject::cast::AsObject for FileInputStream {

    #[inline]
    fn as_gobject_object(&self) -> &gobject::Object {
        self.parent_instance.as_gobject_object()
    }

    #[inline]
    fn as_mut_gobject_object(&mut self) -> &mut gobject::Object {
        self.parent_instance.as_mut_gobject_object()
    }
}
