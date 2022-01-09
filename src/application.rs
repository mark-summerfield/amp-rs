// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    about_html, Action, APPNAME, HELP_HTML, LOAD_ERROR, PAUSE_ICON,
    PLAY_ICON, TICK_TIMEOUT, TINY_TIMEOUT, TOOLBUTTON_SIZE,
};
use crate::html_form;
use crate::mainwindow;
use crate::options_form;
use crate::util;
use fltk::prelude::*;
use soloud::prelude::*;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    play_pause_button: fltk::button::Button,
    info_view: fltk::misc::HelpView,
    volume_slider: fltk::valuator::HorFillSlider,
    volume_label: fltk::frame::Frame,
    time_slider: fltk::valuator::HorFillSlider,
    time_label: fltk::frame::Frame,
    helpform: Option<html_form::Form>,
    player: soloud::Soloud,
    wav: soloud::audio::Wav,
    handle: soloud::Handle,
    playing: bool,
    startup: bool,
    sender: fltk::app::Sender<Action>,
    receiver: fltk::app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        let app =
            fltk::app::App::default().with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let mut widgets = mainwindow::make(sender);
        mainwindow::add_event_handlers(&mut widgets.mainwindow, sender);
        widgets.mainwindow.show();
        let mut player =
            soloud::Soloud::default().expect("Cannot access audio backend");
        player.set_pause_all(true);
        let load = mainwindow::update_widgets_from_config(&mut widgets);
        let mut volume_slider = widgets.volume_slider.clone();
        let mut time_slider = widgets.time_slider.clone();
        let mut app = Self {
            app,
            mainwindow: widgets.mainwindow,
            play_pause_button: widgets.play_pause_button,
            info_view: widgets.info_view,
            volume_slider: widgets.volume_slider,
            volume_label: widgets.volume_label,
            time_slider: widgets.time_slider,
            time_label: widgets.time_label,
            helpform: None,
            player,
            wav: soloud::audio::Wav::default(),
            handle: unsafe { soloud::Handle::from_raw(0) },
            playing: false,
            startup: true,
            sender,
            receiver,
        };
        if load {
            app.load_track();
        }
        #[allow(clippy::clone_on_copy)]
        let sender = sender.clone();
        volume_slider.set_callback(move |_| {
            sender.send(Action::VolumeUpdate);
        });
        #[allow(clippy::clone_on_copy)]
        let sender = sender.clone();
        time_slider.set_callback(move |_| {
            sender.send(Action::TimeUpdate);
        });
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Load => self.on_open(),
                    Action::Previous => self.on_previous(),
                    Action::Replay => self.on_replay(),
                    Action::PlayOrPause => self.on_play_or_pause(),
                    Action::SpacePressed => self.on_space_pressed(),
                    Action::Tick => self.on_tick(),
                    Action::Next => self.on_next(),
                    Action::VolumeDown => self.on_volume_down(),
                    Action::VolumeUp => self.on_volume_up(),
                    Action::VolumeUpdate => self.on_volume_update(),
                    Action::TimeUpdate => self.on_time_update(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    pub fn on_open(&mut self) {
        let mut form = fltk::dialog::FileDialog::new(
            fltk::dialog::FileDialogType::BrowseFile);
        form.set_title(&format!("Choose Track — {}", APPNAME));
        let _ = form.set_directory(&util::get_track_dir()); // Ignore error
        form.set_filter("Audio Files\t*.{flac,mogg,mp3,oga,ogg,wav}");
        form.show();
        let filename = form.filename();
        if filename.exists() {
            {
                let mut config = CONFIG.get().write().unwrap();
                config.track = filename;
                config.pos = 0.0;
            }
            self.load_track();
        }
    }

    fn on_previous(&mut self) {
        let track = {
            let config = CONFIG.get().read().unwrap();
            config.track.clone()
        };
        if let Some(track) =
            util::get_prev_or_next_track(&track, util::WhichTrack::Previous)
        {
            self.auto_play_track(track);
        }
    }

    fn on_replay(&mut self) {
        if self.playing {
            self.on_play_or_pause(); // PAUSE
        }
        {
            let mut config = CONFIG.get().write().unwrap();
            config.pos = 0.0;
        }
        self.seek(0.0);
        self.on_play_or_pause(); // PLAY
    }

    fn on_play_or_pause(&mut self) {
        if self.startup {
            self.at_startup();
        }
        let icon = if self.playing {
            self.player.set_pause(self.handle, true);
            PLAY_ICON
        } else {
            self.player.set_pause(self.handle, false);
            #[allow(clippy::clone_on_copy)]
            let sender = self.sender.clone();
            fltk::app::add_timeout3(TINY_TIMEOUT, move |_| {
                sender.send(Action::Tick);
            });
            PAUSE_ICON
        };
        let mut icon = fltk::image::SvgImage::from_data(icon).unwrap();
        icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
        self.play_pause_button.set_image(Some(icon));
        self.playing = !self.playing;
    }

    fn on_space_pressed(&mut self) {
        self.play_pause_button.set_value(true);
        let mut play_pause_button = self.play_pause_button.clone();
        fltk::app::add_timeout3(TINY_TIMEOUT, move |_| {
            play_pause_button.set_value(false);
            play_pause_button.do_callback();
        });
    }

    fn on_next(&mut self) {
        let track = {
            let config = CONFIG.get().read().unwrap();
            config.track.clone()
        };
        if let Some(track) =
            util::get_prev_or_next_track(&track, util::WhichTrack::Next)
        {
            self.auto_play_track(track);
        }
    }

    fn on_volume_down(&mut self) {
        self.change_volume(
            (self.volume_slider.value() as f32 - 0.05).max(0.0),
        );
    }

    fn on_volume_up(&mut self) {
        self.change_volume(
            (self.volume_slider.value() as f32 + 0.05).min(1.0),
        );
    }

    fn on_volume_update(&mut self) {
        let volume = self.volume_slider.value() as f32;
        self.player.set_volume(self.handle, volume);
        self.volume_label
            .set_label(&format!("{}%", (volume * 100.0).round()));
        fltk::app::redraw(); // redraws the world
    }

    fn on_time_update(&mut self) {
        self.seek(self.time_slider.value());
        fltk::app::redraw(); // redraws the world
    }

    fn on_options(&mut self) {
        options_form::Form::default();
    }

    fn on_about(&mut self) {
        html_form::Form::new(
            "About",
            &about_html(&self.player),
            true,
            480,
            300,
            false,
        );
    }

    fn on_help(&mut self) {
        if let Some(helpform) = &mut self.helpform {
            helpform.show();
        } else {
            self.helpform = Some(html_form::Form::new(
                "Help", HELP_HTML, false, 380, 420, true,
            ));
        }
    }

    fn on_quit(&mut self) {
        let mut config = CONFIG.get().write().unwrap();
        config.window_x = self.mainwindow.x();
        config.window_y = self.mainwindow.y();
        config.window_width = self.mainwindow.width();
        config.window_height = self.mainwindow.height();
        config.volume = self.volume_slider.value();
        config.pos = self.time_slider.value();
        // We already have the track
        config.save();
        self.app.quit();
    }

    fn on_tick(&mut self) {
        if self.playing {
            let pos = self.player.stream_position(self.handle);
            let length = self.wav.length();
            if self.player.voice_count() == 0 {
                // Reached the end
                self.on_next();
                return;
            }
            self.time_slider.set_value(pos);
            self.time_label.set_label(&format!(
                "{}/{}",
                util::humanized_time(pos),
                util::humanized_time(length)
            ));
            fltk::app::redraw(); // redraws the world
            #[allow(clippy::clone_on_copy)]
            let sender = self.sender.clone();
            fltk::app::add_timeout3(TICK_TIMEOUT, move |_| {
                sender.send(Action::Tick);
            });
        }
    }

    fn at_startup(&mut self) {
        let pos = {
            let config = CONFIG.get().read().unwrap();
            config.pos
        };
        self.seek(pos);
        self.playing = false;
        self.startup = false;
    }

    fn load_track(&mut self) {
        if self.playing {
            self.on_play_or_pause(); // PAUSE
            self.player.stop_all();
        }
        let config = CONFIG.get().read().unwrap();
        let message = match self.wav.load(&config.track) {
            Ok(_) => {
                self.handle = self.player.play(&self.wav);
                self.player.set_pause(self.handle, true);
                self.player.set_volume(
                    self.handle,
                    self.volume_slider.value() as f32,
                );
                self.time_slider.set_range(0.0, self.wav.length());
                self.time_slider.set_step(self.wav.length(), 20);
                let pos = if self.startup { config.pos } else { 0.0 };
                self.time_slider.set_value(pos);
                self.time_label.set_label(&format!(
                    "{}/{}",
                    util::humanized_time(pos),
                    util::humanized_time(self.wav.length())
                ));
                util::get_track_data_html(&config.track)
            }
            Err(_) => {
                LOAD_ERROR.replace("FILE", &config.track.to_string_lossy())
            }
        };
        self.info_view.set_value(&message);
        fltk::app::redraw(); // redraws the world
    }

    fn change_volume(&mut self, volume: f32) {
        self.player.set_volume(self.handle, volume);
        self.volume_slider.set_value(volume as f64);
        self.volume_label
            .set_label(&format!("{}%", (volume * 100.0).round()));
        fltk::app::redraw(); // redraws the world
    }

    fn auto_play_track(&mut self, track: std::path::PathBuf) {
        if self.playing {
            self.on_play_or_pause(); // PAUSE
        }
        {
            let mut config = CONFIG.get().write().unwrap();
            config.track = track;
        }
        self.load_track();
        self.on_play_or_pause(); // PLAY
    }

    fn seek(&mut self, pos: f64) {
        if self.player.seek(self.handle, pos).is_ok() {
            while self.player.stream_position(self.handle) < pos {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        self.time_slider.set_value(pos);
        self.time_label.set_label(&format!(
            "{}/{}",
            util::humanized_time(pos),
            util::humanized_time(self.wav.length())
        ));
        fltk::app::redraw(); // redraws the world
    }
}
