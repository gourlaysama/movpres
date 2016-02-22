extern crate mov_pres;
extern crate gtk;
extern crate vlc;
extern crate x11;

use mov_pres::Player;


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

    pl.show_configure();

    gtk::main();
}

// libVLC needs this for best playback (harware acceleration)
// and it needs to happen before gtk::init
fn init_x11() {
    unsafe {
        x11::xlib::XInitThreads();
    }
}
