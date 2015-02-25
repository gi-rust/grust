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

#[macro_export]
macro_rules! g_str {
    ($lit:expr) => {
        unsafe {
            std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr()
                                        as *const $crate::types::gchar)
        }
    }
}

#[macro_export]
macro_rules! g_utf8 {
    ($lit:expr) => {
        $crate::gstr::Utf8::from_static_str(concat!($lit, "\0"))
    }
}

#[macro_export]
macro_rules! g_error_match {
    (
        ($inp:expr) {
            ($slot:ident : $errtype:ty) => $handler:expr,
            $(($slot_tail:ident : $errtype_tail:ty) => $handler_tail:expr,)*
            other $catchall_slot:ident => $catchall_handler:expr
        }
    ) => {
        {
            let err: $crate::error::Error = $inp;
            let res: ::std::result::Result<$errtype, $crate::error::Error>
                     = err.into_domain();
            match res {
                Ok($slot) => $handler,
                Err(e) => g_error_match! {
                    (e) {
                        $(($slot_tail: $errtype_tail) => $handler_tail,)*
                        other $catchall_slot => $catchall_handler
                    }
                }
            }
        }
    };
    (
        ($inp:expr) {
            other $catchall_slot:ident => $catchall_handler:expr
        }
    ) => {
        {
            let $catchall_slot: $crate::error::Error = $inp;
            $catchall_handler
        }
    }
}

#[macro_export]
macro_rules! g_static_quark {
    ($lit:expr) => {
        {
            use $crate::quark::StaticQuark;

            static QUARK: StaticQuark =
                StaticQuark($lit, ::std::sync::atomic::ATOMIC_USIZE_INIT);

            QUARK.get()
        }
    }
}

#[macro_export]
macro_rules! g_type_register_box {
    ($t:ty, $name:expr) => {
        unsafe impl $crate::boxed::BoxRegistered for $t {
            fn box_type() -> $crate::gtype::GType {
                use ::std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
                use ::std::sync::atomic::Ordering::{Acquire,Release};
                use ::std::sync::{Once, ONCE_INIT};

                static REGISTERED: AtomicUsize = ATOMIC_USIZE_INIT;
                static INIT: Once = ONCE_INIT;

                INIT.call_once(|| {
                    let gtype = $crate::boxed::register_box_type::<$t>($name);
                    REGISTERED.store(gtype.to_raw() as usize, Release);
                });

                let raw = REGISTERED.load(Acquire)
                          as $crate::gtype::raw::GType;
                unsafe { $crate::gtype::GType::from_raw(raw) }
            }
        }
    }
}

#[macro_export]
macro_rules! g_impl_boxed_type_for_ref {
    ($t:ty, $get_type:path) => {
        impl $crate::boxed::BoxedType for $crate::refcount::Ref<$t> {

            fn get_type() -> $crate::gtype::GType {
                unsafe { $crate::gtype::GType::from_raw($get_type()) }
            }

            #[inline]
            unsafe fn from_ptr(ptr: $crate::types::gpointer) -> Self {
                let ptr = ptr as *mut <$t as $crate::wrap::Wrapper>::Raw;
                $crate::refcount::Ref::from_raw(ptr)
            }

            #[inline]
            unsafe fn into_ptr(self) -> $crate::types::gpointer {
                $crate::refcount::ref_into_raw(self) as $crate::types::gpointer
            }
        }
    }
}
