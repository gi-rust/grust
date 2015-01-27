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
        $crate::gstr::GStr::from_static_bytes(concat!($lit, "\0").as_bytes())
    }
}

#[macro_export]
macro_rules! g_utf8 {
    ($lit:expr) => {
        $crate::gstr::Utf8::from_static_str(concat!($lit, "\0"))
    }
}

#[macro_export]
macro_rules! g_static_quark {
    ($lit:expr) => {
        {
            use $crate::quark::StaticQuark;

            static QUARK: StaticQuark =
                StaticQuark($lit, std::sync::atomic::ATOMIC_UINT_INIT);

            QUARK.get()
        }
    }
}
