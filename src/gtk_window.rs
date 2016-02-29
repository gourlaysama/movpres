use std::thread;
use std::process;
use std::mem::transmute;

use gdk::Window;
use glib::translate::*;
use gtk;
use ffi;

pub fn gdk_x11_window_get_xid(window: &Window) -> u32 {
    unsafe { ffi::gdk_x11_window_get_xid(window.to_glib_none().0) }
}

pub trait FileChooserButtonSignals {
    fn connect_file_set<F: Fn(gtk::Widget) + 'static>(&self, f: F) -> u64;
}

struct CallbackGuard;

impl Drop for CallbackGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            process::exit(101);
        }
    }
}

macro_rules! callback_guard {
    () => (
        let _guard = CallbackGuard;
    )
}

// libstd stability workaround
unsafe fn into_raw<T>(b: Box<T>) -> *mut T {
    transmute(b)
}

mod filechooserbutton {
    use std::mem::transmute;

    use gtk::Widget;
    use gtk::traits::*;
    use gtk_sys::GtkWidget;
    use glib::signal::connect;
    use super::CallbackGuard;
    use super::into_raw;


    impl<T: FFIWidget + WidgetTrait> super::FileChooserButtonSignals for T {
        fn connect_file_set<F: Fn(Widget) + 'static>(&self, f: F) -> u64 {
            unsafe {
                let f: Box<Box<Fn(Widget) + 'static>> = Box::new(Box::new(f));
                connect(self.unwrap_widget() as *mut _,
                        "file-set",
                        transmute(void_trampoline),
                        into_raw(f) as *mut _)
            }
        }
    }

    extern "C" fn void_trampoline(this: *mut GtkWidget, f: &Box<Fn(Widget) + 'static>) {
        callback_guard!();
        f(FFIWidget::wrap_widget(this));
    }

}
