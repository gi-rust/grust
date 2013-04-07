use gobject::raw::GObject;
use gobject::raw::symbols::{g_object_ref,g_object_unref};

pub struct Object<R> {
    priv wrapped: *R
}

pub unsafe fn wrap_object<R>(obj: *R) -> Object<R> {
    Object { wrapped: obj }
}

impl<R> Object<R> {
    pub unsafe fn unwrap(&self) -> *R { self.wrapped }

    pub unsafe fn get_g_object(&self) -> *GObject {
        cast::transmute(self.wrapped)
    }
}

#[unsafe_destructor]
impl<R> Drop for Object<R> {
    fn finalize(&self) {
        unsafe {
            g_object_unref(self.get_g_object());
        }
    }
}

impl<R> Clone for Object<R> {
    fn clone(&self) -> Object<R> {
        unsafe {
            g_object_ref(self.get_g_object());
            wrap_object(self.wrapped)
        }
    }
}
