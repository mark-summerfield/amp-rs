// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    about_html, Action, APPNAME, HELP_HTML, LOAD_ERROR, ON_LOAD,
    PAUSE_ICON, PLAY_ICON, TICK_TIMEOUT, TOOLBUTTON_SIZE,
    WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
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
        widgets.mainwindow.size_range(
            WINDOW_WIDTH_MIN,
            WINDOW_HEIGHT_MIN,
            1024,
            800,
        );
        widgets.mainwindow.show();
        let mut player =
            soloud::Soloud::default().expect("Cannot access audio backend");
        player.set_pause_all(true);
        let load;
        {
            let config = CONFIG.get().read().unwrap();
            load = config.track.exists();
            widgets.volume_slider.set_value(config.volume);
            widgets.volume_label.set_label(&format!(
                "{}%",
                (config.volume * 100.0).round()
            ));
        }
        let mut volume_slider = widgets.volume_slider.clone();
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
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    pub fn on_open(&mut self) {
        if let Some(filename) = fltk::dialog::file_chooser(
            &format!("Choose Track — {}", APPNAME),
            "Audio Files (*.{oga,ogg,mp3})",
            &util::get_track_dir(),
            false,
        ) {
            {
                let mut config = CONFIG.get().write().unwrap();
                config.track = std::path::PathBuf::from(filename);
                config.pos = 0.0;
            }
            self.load_track();
        }
    }

    fn on_previous(&mut self) {
        dbg!("Previous"); // TODO
    }

    fn on_replay(&mut self) {
        dbg!("Replay"); // TODO
    }

    fn on_play_or_pause(&mut self) {
        let icon = if self.playing {
            dbg!("PAUSE");
            self.player.pause(self.handle);
            PLAY_ICON
        } else {
            dbg!("PLAY");
            self.handle = self.player.play(&self.wav);
            self.player
                .set_volume(self.handle, self.volume_slider.value() as f32);
            #[allow(clippy::clone_on_copy)]
            let sender = self.sender.clone();
            fltk::app::add_timeout(TICK_TIMEOUT, move || {
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
        fltk::app::add_timeout(0.075, move || {
            play_pause_button.set_value(false);
            play_pause_button.do_callback();
        });
    }

    fn on_next(&mut self) {
        dbg!("Next"); // TODO
    }

    fn on_volume_down(&mut self) {
        let volume = (self.volume_slider.value() as f32 - 0.05).max(0.0);
        self.player.set_volume(self.handle, volume);
        self.volume_slider.set_value(volume as f64);
        self.volume_label
            .set_label(&format!("{}%", (volume * 100.0).round()));
    }

    fn on_volume_up(&mut self) {
        let volume = (self.volume_slider.value() as f32 + 0.05).min(1.0);
        self.player.set_volume(self.handle, volume);
        self.volume_slider.set_value(volume as f64);
        self.volume_label
            .set_label(&format!("{}%", (volume * 100.0).round()));
    }

    fn on_volume_update(&mut self) {
        let volume = self.volume_slider.value() as f32;
        self.player.set_volume(self.handle, volume);
        self.volume_label
            .set_label(&format!("{}%", (volume * 100.0).round()));
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
            self.time_slider
                .set_value(self.player.stream_time(self.handle));
            self.time_label.set_label(&format!(
                "{}″/{}″",
                self.player.stream_time(self.handle).round(),
                self.wav.length().round()
            ));
            fltk::app::redraw(); // redraws the world
            #[allow(clippy::clone_on_copy)]
            let sender = self.sender.clone();
            fltk::app::add_timeout(TICK_TIMEOUT, move || {
                sender.send(Action::Tick);
            });
        }
    }

    fn load_track(&mut self) {
        let config = CONFIG.get().read().unwrap();
        let message = match self.wav.load(&config.track) {
            Ok(_) => {
                self.time_slider.set_range(0.0, self.wav.length());
                self.time_slider.set_step(self.wav.length(), 20);
                self.time_label.set_label(&format!(
                    "0″/{}″",
                    self.wav.length().round()
                ));
                ON_LOAD.replace("FILE", &config.track.to_string_lossy())
            }
            Err(_) => {
                LOAD_ERROR.replace("FILE", &config.track.to_string_lossy())
            }
        };
        self.info_view.set_value(&message);
    }
}
