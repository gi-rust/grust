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

use std::mem;

pub unsafe trait Wrapper {

    type Raw : Sized;

    #[inline]
    fn as_ptr<'a>(&'a self) -> *const <Self as Wrapper>::Raw {
        self as *const Self as *const <Self as Wrapper>::Raw
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut <Self as Wrapper>::Raw {
        self as *mut Self as *mut <Self as Wrapper>::Raw
    }
}

#[inline]
pub unsafe fn from_raw<'a, T, U>(ptr: *const <T as Wrapper>::Raw,
                                 life_anchor: &'a U)
                                -> &'a T
    where T: Wrapper
{
    mem::copy_lifetime(life_anchor, &*(ptr as *const T))
}

#[inline]
pub unsafe fn from_raw_mut<'a, T, U>(ptr: *mut <T as Wrapper>::Raw,
                                     _life_anchor: &'a U)
                                    -> &'a mut T
    where T: Wrapper
{
    mem::transmute(&*(ptr as *mut T))
}
