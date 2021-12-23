// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    about_html, Action, HELP_HTML, WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::html_form;
use crate::mainwindow;
use crate::options_form;
use fltk::prelude::*;

pub struct Application {
    app: fltk::app::App,
    mainwindow: fltk::window::Window,
    play_pause_button: fltk::button::Button,
    info_view: fltk::misc::HelpView,
    volume_slider: fltk::valuator::HorNiceSlider,
    volume_label: fltk::frame::Frame,
    time_slider: fltk::valuator::HorNiceSlider,
    time_label: fltk::frame::Frame,
    helpform: Option<html_form::Form>,
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
        Self {
            app,
            mainwindow: widgets.mainwindow,
            play_pause_button: widgets.play_pause_button,
            info_view: widgets.info_view,
            volume_slider: widgets.volume_slider,
            volume_label: widgets.volume_label,
            time_slider: widgets.time_slider,
            time_label: widgets.time_label,
            helpform: None,
            receiver,
        }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::Load => self.on_open(),
                    Action::Previous => {
                        dbg!("Previous"); // TODO
                    }
                    Action::Replay => {
                        dbg!("Replay"); // TODO
                    }
                    Action::PlayOrPause => {
                        dbg!("PlayOrPause"); // TODO
                    }
                    Action::Next => {
                        dbg!("Next"); // TODO
                    }
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    pub fn on_open(&mut self) {
        dbg!("on_open");
    }

    fn on_options(&mut self) {
        options_form::Form::default();
    }

    fn on_about(&mut self) {
        html_form::Form::new("About", &about_html(), true, 450, 300, false);
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
}
