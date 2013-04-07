use grust::types::*;
use glib::raw::symbols::{g_free, g_strdup};

pub struct GStr {
    priv data: *gchar,
}

impl GStr {
    pub unsafe fn wrap(data: *gchar) -> GStr { GStr{ data: data } }
}

impl Drop for GStr {
    fn finalize(&self) {
        unsafe {
            g_free(cast::transmute(self.data));
        }
    }
}

impl Clone for GStr {
    fn clone(&self) -> GStr {
        unsafe {
            GStr::wrap(g_strdup(self.data))
        }
    }
}

impl ToStr for GStr {
    fn to_str(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(self.data)
        }
    }
}

impl Eq for GStr {
    fn eq(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }

    fn ne(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) != 0
        }
    }
}

impl TotalEq for GStr {
    fn equals(&self, other: &GStr) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }
}
