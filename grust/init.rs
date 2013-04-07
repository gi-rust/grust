use gobject::raw::symbols::g_type_init;

pub fn init() {
    unsafe {
        g_type_init();
    }
}
