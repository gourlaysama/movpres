extern crate gdk_sys;
extern crate gdk;
extern crate gtk;
extern crate gtk_sys;
extern crate glib;
extern crate vlc;

use std::sync::Arc;

use gtk::traits::*;
use gtk::Window;
use gtk::widgets::Builder;
use gtk::signal::Inhibit;
use vlc::MediaPlayerVideoEx;
use vlc::MediaPlayerAudioEx;
use vlc::State;
use gtk_window::FileChooserButtonSignals;

mod ffi;
pub mod gtk_window;

// macro for getting stuff from a builder
macro_rules! builder_get {
    ( $b:expr, $name:expr, $typ:ty ) => {
        unsafe { $b.get_object::<$typ>($name).expect(&format!("cannot find '{}'.'", $name)) }
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
        let glade_src = include_str!("windows.glade");
        let builder = Builder::new_from_string(glade_src).expect("can't find builder");

        let play_window = builder_get!(&builder, "play_window", Window);
        let configure_window = builder_get!(&builder, "configure_window", Window);
        let control_window = builder_get!(&builder, "control_window", Window);

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

        p.play_window.connect_show({
            let pp = p.clone();
            move |_| {
                let id = gtk_window::gdk_x11_window_get_xid(&pp.play_window.get_window().unwrap());
                pp.media_player.set_xwindow(id);
            }
        });

        p.control_window.connect_show({
            let pp = p.clone();
            move |_| {
                let media = pp.media_player.get_media().unwrap();
                let path = media.mrl().unwrap();
                builder_get!(&pp.builder, "label_file", gtk::Label).set_text(&path);
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

        builder_get!(&p.builder, "button_file", gtk::Widget).connect_file_set({
            let pp = p.clone();
            move |_| {
                let path = builder_get!(&pp.builder, "button_file", gtk::FileChooserDialog)
                               .get_filename()
                               .unwrap();
                let md = vlc::Media::new_path(&pp.vlc_instance, &path).unwrap();
                pp.media_player.set_media(&md);

                builder_get!(&pp.builder, "button_start", gtk::Button).set_sensitive(true);
            }
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

        builder_get!(&p.builder, "button_quit", gtk::Button).connect_clicked(|_| {
            gtk::main_quit();
        });

        builder_get!(&p.builder, "button_play", gtk::Button).connect_clicked({
            let pp = p.clone();
            move |_| {
                match pp.media_player.state() {
                    State::Playing | State::Paused => pp.media_player.pause(),
                    _ => pp.media_player.play().unwrap(),
                };
                builder_get!(&pp.builder, "button_play", gtk::Button)
                    .set_image(&gtk::Image::new_from_icon_name(if pp.media_player.is_playing() {
                                                                   "media-playback-start"
                                                               } else {
                                                                   "media-playback-pause"
                                                               },
                                                               4)
                                    .unwrap());
            }
        });

        builder_get!(&p.builder, "button_stop", gtk::Button).connect_clicked({
            let pp = p.clone();
            move |_| {
                pp.media_player.stop();
                builder_get!(&pp.builder, "button_play", gtk::Button)
                    .set_image(&gtk::Image::new_from_icon_name("media-playback-start", 4).unwrap());
            }
        });

        let volume_adjustment = gtk::Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 10.0).unwrap();
        builder_get!(&p.builder, "button_volume", gtk::VolumeButton)
            .set_adjustment(&volume_adjustment);

        volume_adjustment.connect_value_changed({
            let pp = p.clone();
            move |a| {
                let volume = a.get_value();
                pp.media_player.set_volume(volume as i32).unwrap();
            }
        });
        volume_adjustment.set_value(50.0);

        builder_get!(&p.builder, "button_fullscreen", gtk::Button).connect_clicked({
            let pp = p.clone();
            move |_| {
                pp.media_player.toggle_fullscreen();
            }
        });

        p
    }

    pub fn set_media(&self, path: &str) {
        builder_get!(&self.builder, "button_file", gtk::FileChooserDialog).set_filename(path);
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

    pub fn show_configure(&self) {
        self.configure_window.show_all();
    }
}
