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
use ll::*;
use plumbing::GMainLoop;

pub struct EventLoop {
    priv raw: *GMainLoop
}

impl EventLoop {
    pub fn new() -> EventLoop {
        unsafe {
            EventLoop { raw: grustna_main_loop_new_thread_local() }
        }
    }

    pub fn run(&self) {
        unsafe {
            grustna_main_loop_run_thread_local(self.raw);
        }
    }

    pub fn quit(&self) {
        unsafe {
            g_main_loop_quit(self.raw);
        }
    }
}

#[unsafe_destructor]
impl Drop for EventLoop {
    fn finalize(&self) {
        unsafe {
            g_main_loop_unref(self.raw);
        }
    }
}

impl Clone for EventLoop {
    fn clone(&self) -> EventLoop {
        unsafe {
            g_main_loop_ref(self.raw);
            EventLoop { raw: self.raw }
        }
    }
}
