extern crate mov_pres;
extern crate gtk;
extern crate vlc;
extern crate x11;

use mov_pres::Player;
use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
  init_x11();
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

  let pl = Player::new();

  pl.window.set_title("Presenter");
  pl.window.set_default_size(850, 670);
  pl.window.set_window_position(gtk::WindowPosition::Center);
  pl.window.show_all();

  pl.set_media(path);

  let control_window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
  control_window.set_title("Controller");
  control_window.set_default_size(200, 300);

  let play_button = gtk::Button::new_with_label("Play/Pause").unwrap();

  play_button.connect_clicked({
      let ppl = pl.clone();
      move |_| {
          ppl.toggle_play();
  }});
  control_window.add(&play_button);

  control_window.show_all();

  pl.play();

  pl.window.connect_delete_event({
      let ppl = pl.clone();
      move |_, _| {
        ppl.stop();

        gtk::main_quit();
        Inhibit(false)
  }});

  gtk::main();
}

// libVLC needs this for best playback (harware acceleration)
fn init_x11() {
    unsafe {
        x11::xlib::XInitThreads();
    }
}
