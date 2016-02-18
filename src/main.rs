extern crate mov_pres;
extern crate gtk;
extern crate vlc;
extern crate x11;

use mov_pres::Player;
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


  let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
  window.set_title("Presenter");
  window.set_default_size(850, 670);
  window.set_visible(false);
  window.show_all();

  let pl = Player::new(&window);

  pl.set_media(path);
  pl.play();

  window.connect_delete_event(move |_, _| {
      pl.stop();

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
