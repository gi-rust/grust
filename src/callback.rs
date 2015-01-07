// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

use types::gpointer;

use std::mem::transmute;

trait AsyncClosure<Args, Ret> {
    fn invoke(self: Box<Self>, args: Args) -> Ret;
}

impl<Args, Ret, F> AsyncClosure<Args, Ret> for F
    where F: FnOnce<Args, Ret> + Send
{
    fn invoke(self: Box<F>, args: Args) -> Ret {
        (*self).call_once(args)
    }
}

pub struct AsyncCallback<Args, Ret> {
    closure: Box<AsyncClosure<Args, Ret> + Send>
}

impl<Args, Ret> AsyncCallback<Args, Ret> {

    pub fn new<F>(func: Box<F>) -> AsyncCallback<Args, Ret>
        where F: FnOnce<Args, Ret> + Send
    {
        AsyncCallback { closure: func }
    }

    pub fn invoke(self, args: Args) -> Ret {
        self.closure.invoke(args)
    }

    pub unsafe fn into_raw_ptr(self) -> gpointer {
        transmute(box self)
    }
}

pub unsafe fn invoke<Args, Ret>(ptr: gpointer, args: Args) -> Ret {
    let b: Box<AsyncCallback<Args, Ret>> = transmute(ptr);
    b.invoke(args)
}
