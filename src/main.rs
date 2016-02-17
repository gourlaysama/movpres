extern crate mov_pres;
extern crate gtk;
extern crate vlc;
extern crate x11;

use mov_pres::gtk_window;
use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
  if gtk::init().is_err() {
      println!("Failed to initialize GTK. Exiting.");
      return;
  }

  let args: Vec<String> = std::env::args().collect();
  let path = match args.get(1) {
      Some(s) => s,
      None => {
          println!("Usage: movpres <path_to_media_file>");
          return;
      }
  };

  init_x11();

  let instance = vlc::Instance::new().unwrap();

  let md = vlc::Media::new_path(&instance, path).unwrap();
  let mdp = vlc::MediaPlayer::new(&instance).unwrap();

  mdp.set_media(&md);

  let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
  window.set_title("Presenter");
  window.set_default_size(850, 670);
  window.set_visible(false);
  window.show_all();

  let id = gtk_window::gdk_x11_window_get_xid(&window.get_window().unwrap());
  mdp.set_xwindow(id);
  mdp.play().unwrap();

  window.connect_delete_event(move |_, _| {
      mdp.stop();

      gtk::main_quit();
      Inhibit(false)
  });

  window.set_visible(true);

  gtk::main();
}

// libVLC needs this for best playback (harware acceleration)
fn init_x11() {
    unsafe {
        x11::xlib::XInitThreads();
    }
}
