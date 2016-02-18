use gdk_sys::GdkWindow;

extern "C" {
    // x11 specific
    pub fn gdk_x11_window_get_xid(window: *mut GdkWindow) -> u32;
}
