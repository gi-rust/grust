pub mod raw {
    use grust::types::*;

    #[link_name="glib-2.0"]
    pub extern mod symbols {
        pub fn g_free(mem: *());
        pub fn g_strdup(str: *gchar) -> *gchar;
    }
}
