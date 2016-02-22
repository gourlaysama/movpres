extern crate mov_pres;
extern crate gtk;
extern crate vlc;
extern crate x11;

use mov_pres::Player;
use gtk::traits::*;


fn main() {
    init_x11();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK. Exiting.");
        return;
    }

    let pl = Player::new();

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(s) => pl.set_media(&s),
        None => (),
    };

    pl.configure();

    //    let control_window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    // control_window.set_title("Controller");
    // control_window.set_default_size(100, 32);
    //
    // let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    // let play_image = gtk::Image::new_from_icon_name("media-playback-start",
    // gtk::IconSize::Button as i32)
    // .unwrap();
    // let play_button = gtk::Button::new().unwrap();
    // play_button.add(&play_image);
    //
    // let stop_image = gtk::Image::new_from_icon_name("media-playback-stop",
    // gtk::IconSize::Button as i32)
    // .unwrap();
    // let stop_button = gtk::Button::new().unwrap();
    // stop_button.add(&stop_image);
    //
    // let fullscreen_button = gtk::ToggleButton::new_with_label("Fullscreen").unwrap();
    //
    // buttons.pack_start(&play_button, false, false, 0);
    // buttons.pack_start(&stop_button, false, false, 0);
    // buttons.pack_start(&fullscreen_button, false, false, 0);
    //
    // play_button.connect_clicked({
    // let ppl = pl.clone();
    // move |_| {
    // ppl.toggle_play();
    // }
    // });
    // stop_button.connect_clicked({
    // let ppl = pl.clone();
    // move |_| {
    // ppl.stop();
    // }
    // });
    // fullscreen_button.connect_clicked({
    // let ppl = pl.clone();
    // let fb = fullscreen_button.clone();
    // move |_| {
    // let fs = fb.get_active();
    // ppl.set_fullscreen(fs);
    // }
    // });
    // buttons.show_all();
    //
    // control_window.add(&buttons);
    // control_window.show_all();
    //
    // pl.play();
    //
    gtk::main();
}

// libVLC needs this for best playback (harware acceleration)
// and it needs to happen before gtk::init
fn init_x11() {
    unsafe {
        x11::xlib::XInitThreads();
    }
}
