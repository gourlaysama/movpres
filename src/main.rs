extern crate gtk;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
  if gtk::init().is_err() {
      println!("Failed to initialize GTK. Exiting.");
      return;
  }

  let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
  window.set_title("Movie Presenter");
  window.set_default_size(350, 70);

  window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
  });

  window.show_all();
  gtk::main();
}
