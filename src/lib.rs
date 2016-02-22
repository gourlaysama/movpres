extern crate gdk_sys;
extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate vlc;

use std::sync::Arc;

use gtk::traits::*;
use gtk::Window;
use gtk::widgets::Builder;
use gtk::signal::Inhibit;
use vlc::MediaPlayerVideoEx;

mod ffi;
pub mod gtk_window;

// macro for getting stuff from a builder
macro_rules! builder_get {
    ( $b:expr, $name:expr, $typ:ty ) => {
        unsafe { $b.get_object::<$typ>($name).expect("$name") }
    }
}

pub struct Player {
    vlc_instance: vlc::Instance,
    media_player: vlc::MediaPlayer,
    builder: gtk::widgets::Builder,
    play_window: gtk::Window,
    configure_window: gtk::Window,
    control_window: gtk::Window,
}

impl Player {
    pub fn new() -> Arc<Player> {
        let glade_src = include_str!("config_window.glade");
        let builder = Builder::new_from_string(glade_src).expect("can't find builder");

        let play_window = gtk::Window::new(gtk::WindowType::Toplevel).expect("play_window");
        let configure_window = builder_get!(&builder, "configure_window", Window);
        let control_window = gtk::Window::new(gtk::WindowType::Toplevel).expect("control_window");

        let instance = vlc::Instance::new().unwrap();
        let mdp = vlc::MediaPlayer::new(&instance).unwrap();

        let p = Arc::new(Player {
            vlc_instance: instance,
            media_player: mdp,
            builder: builder,
            play_window: play_window,
            configure_window: configure_window,
            control_window: control_window,
        });

        p.style_windows();

        p.play_window.connect_show({
            let pp = p.clone();
            move |_| {
                let id = gtk_window::gdk_x11_window_get_xid(&pp.play_window.get_window().unwrap());
                pp.media_player.set_xwindow(id);
            }
        });

        p.control_window.connect_delete_event({
            let pp = p.clone();
            move |_, _| {
                pp.media_player.stop();

                gtk::main_quit();
                Inhibit(false)
            }
        });

        p.play_window.connect_delete_event({
            let pp = p.clone();
            move |_, _| {
                pp.media_player.stop();
                Inhibit(false)
            }
        });

        p.configure_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        builder_get!(&p.builder, "button_start", gtk::Button).connect_clicked({
            let pp = p.clone();
            move |_| {
                pp.configure_window.hide();
                pp.play_window.show_all();
                pp.control_window.show_all();
                pp.media_player.play().unwrap();
            }
        });

        builder_get!(&p.builder, "button_quit", gtk::Button).connect_clicked({
            let pp = p.clone();
            move |_| {
                gtk::main_quit();
            }
        });

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
        if self.media_player.is_playing() {
            self.media_player.pause();
        } else {
            self.media_player.play().unwrap();
        }
    }

    pub fn set_fullscreen(&self, fullscren: bool) {
        self.media_player.set_fullscreen(fullscren);
    }

    pub fn configure(&self) {
        self.configure_window.show_all();
    }

    // internal stuff

    fn style_windows(&self) {
        self.play_window.set_title("Presenter Output");
        self.play_window.set_default_size(850, 670);
        self.play_window.set_window_position(gtk::WindowPosition::Center);

        self.control_window.set_title("Controller");
        self.control_window.set_default_size(100, 32);

    }
}
