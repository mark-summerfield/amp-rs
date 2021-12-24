// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    about_html, Action, HELP_HTML, LOAD_ERROR, WINDOW_HEIGHT_MIN,
    WINDOW_WIDTH_MIN,
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
            player: soloud::Soloud::default()
                .expect("Cannot access audio backend"),
            wav: soloud::audio::Wav::default(),
            sender,
            receiver,
        };
        let load;
        {
            let config = CONFIG.get().read().unwrap();
            load = config.track.exists();
        }
        if load {
            app.load_track();
        }
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
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    pub fn on_open(&mut self) {
        // "Audio Files\t*.{oga,ogg,mp3}", FIXME This doesn't work right
        if let Some(filename) = fltk::dialog::file_chooser(
            "Choose Track — AMP",
            "*.{oga,ogg,mp3}",
            &util::get_track_dir().to_string_lossy(),
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
        dbg!("Play or Pause"); // TODO
                               /*
                               self.player.play(&self.wav);
                               let sender = self.sender.clone();
                               fltk::app::add_timeout(0.2, move || {
                                   sender.send(Action::Tick);
                               });
                               */
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
        dbg!("on_volume_down"); // TODO
    }

    fn on_volume_up(&mut self) {
        dbg!("on_volume_up"); // TODO
    }

    fn on_options(&mut self) {
        options_form::Form::default();
    }

    fn on_about(&mut self) {
        html_form::Form::new("About", &about_html(), true, 480, 300, false);
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
        let config = CONFIG.get().read().unwrap();
        config.save(
            self.mainwindow.x(),
            self.mainwindow.y(),
            self.mainwindow.width(),
            self.mainwindow.height(),
        );
        self.app.quit();
    }

    fn on_tick(&mut self) {
        dbg!("on_tick");
        if self.player.voice_count() > 0 {}
    }

    fn load_track(&mut self) {
        let config = CONFIG.get().read().unwrap();
        match self.wav.load(&config.track) {
            Ok(_) => {
                dbg!("load_track"); // TODO
            }
            Err(_) => self.info_view.set_value(
                &LOAD_ERROR
                    .replace("FILE", &config.track.to_string_lossy()),
            ),
        }
    }
}
