// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::Action;
use crate::html_form;
use crate::main_window;
use fltk::prelude::*;
use soloud::prelude::*;

pub struct Application {
    pub(crate) app: fltk::app::App,
    pub(crate) main_window: fltk::window::Window,
    pub(crate) play_pause_button: fltk::button::Button,
    pub(crate) history_menu_button: fltk::menu::MenuButton,
    pub(crate) bookmarks_menu_button: fltk::menu::MenuButton,
    pub(crate) info_view: fltk::misc::HelpView,
    pub(crate) volume_slider: fltk::valuator::HorFillSlider,
    pub(crate) volume_label: fltk::frame::Frame,
    pub(crate) time_slider: fltk::valuator::HorFillSlider,
    pub(crate) time_label: fltk::frame::Frame,
    pub(crate) helpform: Option<html_form::Form>,
    pub(crate) player: soloud::Soloud,
    pub(crate) wav: soloud::audio::Wav,
    pub(crate) handle: soloud::Handle,
    pub(crate) playing: bool,
    pub(crate) first_to_play: bool,
    pub(crate) sender: fltk::app::Sender<Action>,
    pub(crate) receiver: fltk::app::Receiver<Action>,
}

impl Application {
    pub fn new() -> Self {
        let app =
            fltk::app::App::default().with_scheme(fltk::app::Scheme::Gleam);
        let (sender, receiver) = fltk::app::channel::<Action>();
        let mut widgets = main_window::make(sender);
        main_window::add_event_handlers(&mut widgets.main_window, sender);
        widgets.main_window.show();
        let mut player =
            soloud::Soloud::default().expect("Cannot access audio backend");
        player.set_pause_all(true);
        let load = main_window::update_widgets_from_config(&mut widgets);
        let mut volume_slider = widgets.volume_slider.clone();
        let mut time_slider = widgets.time_slider.clone();
        let app = Self {
            app,
            main_window: widgets.main_window,
            play_pause_button: widgets.play_pause_button,
            history_menu_button: widgets.history_menu_button,
            bookmarks_menu_button: widgets.bookmarks_menu_button,
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
            first_to_play: true,
            sender,
            receiver,
        };
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
        if load {
            #[allow(clippy::clone_on_copy)]
            let sender = sender.clone();
            fltk::app::add_timeout3(0.01, move |_| {
                sender.send(Action::OnStartup);
            });
        }
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::OnStartup => self.on_startup(),
                    Action::Load => self.on_open(),
                    Action::Previous => self.on_previous(),
                    Action::Replay => self.on_replay(),
                    Action::PlayOrPause => self.on_play_or_pause(),
                    Action::LoadHistoryTrack => {
                        self.on_load_history_track()
                    }
                    Action::LoadBookmarkedTrack => {
                        println!("LoadBookmarkedTrack") // TODO
                    }
                    Action::SpacePressed => self.on_space_pressed(),
                    Action::Tick => self.on_tick(),
                    Action::Next => self.on_next(),
                    Action::VolumeDown => self.on_volume_down(),
                    Action::VolumeUp => self.on_volume_up(),
                    Action::VolumeUpdate => self.on_volume_update(),
                    Action::TimeUpdate => self.on_time_update(),
                    Action::AddToHistory => self.on_add_to_history(),
                    Action::AddBookmark => println!("AddBookmark"), // TODO
                    Action::DeleteBookmark => println!("DeleteBookmark"),     // TODO
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }
}
