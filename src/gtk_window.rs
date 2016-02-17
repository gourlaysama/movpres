use gdk::Window;
use glib::translate::*;
use ffi;


pub fn gdk_x11_window_get_xid(window: &Window) -> u32 {
  unsafe { ffi::gdk_x11_window_get_xid(window.to_glib_none().0) }
}
