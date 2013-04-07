pub mod raw {
    use grust::types::*;

    struct GType(gsize);

    struct GTypeInstance {
        g_class: *(),
    }

    pub struct GObject {
        g_type_instance: GTypeInstance,
        ref_count      : guint,
        data           : *()
    }

    #[link_name="gobject-2.0"]
    pub extern mod symbols {
        fn g_type_init();
        fn g_object_ref(obj: *GObject) -> *();
        fn g_object_unref(obj: *GObject) -> *();
    }
}
