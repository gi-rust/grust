use types::*;

#[link_name="grustna"]
pub extern {
    fn grustna_call(func: *u8,
                    data: *(),
                    context: *GMainContext) -> gboolean;
    fn grustna_main_loop_new_thread_local() -> *GMainLoop;
    fn grustna_main_loop_run_thread_local(l: *GMainLoop);
}

#[link_name="glib-2.0"]
pub extern {
    fn g_free(mem: *());
    fn g_strdup(str: *gchar) -> *gchar;
    fn g_main_context_ref(context: *GMainContext) -> *GMainContext;
    fn g_main_context_unref(context: *GMainContext);
    fn g_main_loop_ref(l: *GMainLoop) -> *GMainLoop;
    fn g_main_loop_unref(l: *GMainLoop);
    fn g_main_loop_run(l: *GMainLoop);
    fn g_main_loop_quit(l: *GMainLoop);
}

#[link_name="gobject-2.0"]
pub extern {
    fn g_type_init();
    fn g_object_ref(obj: *()) -> *();
    fn g_object_unref(obj: *()) -> *();
    fn g_type_check_instance_is_a(instance   : *GTypeInstance,
                                  iface_type : GType) -> gboolean;
    fn g_type_name(t: GType) -> *gchar;
}
