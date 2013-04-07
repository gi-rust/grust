use grust;
use grust::plumbing;

pub mod raw {
    use grust::types::*;

    pub trait GFile { }

    #[link_name="gio-2.0"]
    pub extern mod symbols {
        fn g_file_new_for_path(path: *gchar) -> *GFile;
        fn g_file_get_path(file: *GFile) -> *gchar;
    }
}

pub trait File {
    fn get_path(&self) -> grust::GStr;
}

impl File {
    pub fn new_for_path(path: &str) -> plumbing::Object<raw::GFile> {
        unsafe {
            plumbing::wrap_object(str::as_c_str(path,
                    raw::symbols::g_file_new_for_path))
        }
    }
}

impl File for plumbing::Object<raw::GFile> {
    fn get_path(&self) -> grust::GStr {
        unsafe {
            let ret = raw::symbols::g_file_get_path(self.unwrap());
            grust::GStr::wrap(ret)
        }
    }
}
