extern crate gdk_sys;
extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate vlc;

use gtk::traits::*;

mod ffi;
pub mod gtk_window;

pub struct Player {
    vlc_instance: vlc::Instance,
    media_player: vlc::MediaPlayer,
}

impl Player {
    pub fn new(window: &gtk::Window) -> Player {
        let instance = vlc::Instance::new().unwrap();
        let mdp = vlc::MediaPlayer::new(&instance).unwrap();

        let id = gtk_window::gdk_x11_window_get_xid(&window.get_window().unwrap());
        mdp.set_xwindow(id);

        Player { vlc_instance: instance, media_player: mdp}
    }

    pub fn set_media(&self, path: &str) {
        let md = vlc::Media::new_path(&self.vlc_instance, path).unwrap();
        self.media_player.set_media(&md);
    }

    pub fn play(&self) {
        self.media_player.play().unwrap();
    }

    pub fn stop(&self) {
        self.media_player.stop();
    }
}
