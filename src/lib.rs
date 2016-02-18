extern crate gdk_sys;
extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate vlc;

use std::sync::Arc;

use gtk::traits::*;
use gtk::Window;

mod ffi;
pub mod gtk_window;

pub struct Player {
    vlc_instance: vlc::Instance,
    media_player: vlc::MediaPlayer,
    pub window: gtk::Window,
}

impl Player {
    pub fn new() -> Arc<Player> {
        let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
        let instance = vlc::Instance::new().unwrap();
        let mdp = vlc::MediaPlayer::new(&instance).unwrap();

        let p = Arc::new(Player { vlc_instance: instance, media_player: mdp, window: window });

        p.window.connect_show({
            let pp = p.clone();
            move |_| {
            let id = gtk_window::gdk_x11_window_get_xid(&pp.window.get_window().unwrap());
            pp.media_player.set_xwindow(id);
        }});

        p
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

    pub fn toggle_play(&self) {
        if self.media_player.is_playing() { self.media_player.pause(); } else {
            self.media_player.play().unwrap();
        }
    }
}
