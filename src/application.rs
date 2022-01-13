// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::Action;
use crate::html_form;
use crate::main_window;
use fltk::prelude::*;
use soloud::prelude::*;

pub struct Application {
    pub(crate) app: fltk::app::App,
    pub(crate) main_window: fltk::window::Window,
    pub(crate) prev_button: fltk::button::Button,
    pub(crate) replay_button: fltk::button::Button,
    pub(crate) play_pause_button: fltk::button::Button,
    pub(crate) next_button: fltk::button::Button,
    pub(crate) history_menu_button: fltk::menu::MenuButton,
    pub(crate) bookmarks_menu_button: fltk::menu::MenuButton,
    pub(crate) add_bookmark_button: fltk::button::Button,
    pub(crate) delete_bookmark_button: fltk::button::Button,
    pub(crate) menu_button: fltk::menu::MenuButton,
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
        let mut app = Self {
            app,
            main_window: widgets.main_window,
            prev_button: widgets.prev_button,
            replay_button: widgets.replay_button,
            play_pause_button: widgets.play_pause_button,
            next_button: widgets.next_button,
            history_menu_button: widgets.history_menu_button,
            bookmarks_menu_button: widgets.bookmarks_menu_button,
            add_bookmark_button: widgets.add_bookmark_button,
            delete_bookmark_button: widgets.delete_bookmark_button,
            menu_button: widgets.menu_button,
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
        } else {
            app.update_ui();
        }
        app
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::OnStartup => self.on_startup(),
                    Action::OnBookmarkMenu => self.on_bookmark_menu(),
                    Action::OnHistoryMenu => self.on_history_menu(),
                    Action::OnMenuMenu => self.on_menu_menu(),
                    Action::Load => self.on_open(),
                    Action::Previous => self.on_previous(),
                    Action::Replay => self.on_replay(),
                    Action::PlayOrPause => self.on_play_or_pause(),
                    Action::LoadHistoryTrack => {
                        self.on_load_history_track()
                    }
                    Action::LoadBookmarkedTrack => {
                        self.on_load_bookmarked_track()
                    }
                    Action::SpacePressed => self.on_space_pressed(),
                    Action::Tick => self.on_tick(),
                    Action::MainMenu => {
                        self.menu_button.popup();
                    }
                    Action::Next => self.on_next(),
                    Action::VolumeDown => self.on_volume_down(),
                    Action::VolumeUp => self.on_volume_up(),
                    Action::VolumeUpdate => self.on_volume_update(),
                    Action::TimeUpdate => self.on_time_update(),
                    Action::AddToHistory => self.on_add_to_history(),
                    Action::AddBookmark => self.on_add_bookmark(),
                    Action::DeleteBookmark => self.on_delete_bookmark(),
                    Action::Options => self.on_options(),
                    Action::About => self.on_about(),
                    Action::Help => self.on_help(),
                    Action::Quit => self.on_quit(),
                }
            }
        }
    }

    pub fn populate_history_menu_button(&mut self) {
        main_window::populate_history_menu_button(
            &mut self.history_menu_button,
            self.sender,
        );
        self.update_ui();
    }

    pub fn populate_bookmarks_menu_button(&mut self) {
        main_window::populate_bookmarks_menu_button(
            &mut self.bookmarks_menu_button,
            self.sender,
        );
        self.update_ui();
    }

    pub fn update_ui(&mut self) {
        let (has_track, has_history, has_bookmarks) = {
            let config = CONFIG.get().read().unwrap();
            (
                config.track.exists(),
                config.history.len() > 0,
                config.bookmarks.len() > 0,
            )
        };
        let button_trigger = fltk::enums::CallbackTrigger::Release;
        let no_trigger = fltk::enums::CallbackTrigger::Never;
        let trigger = if has_track { button_trigger } else { no_trigger };
        self.prev_button.set_trigger(trigger);
        self.replay_button.set_trigger(trigger);
        self.play_pause_button.set_trigger(trigger);
        self.next_button.set_trigger(trigger);
        if has_track {
            self.time_slider
                .set_trigger(fltk::enums::CallbackTrigger::Changed);
        } else {
            self.time_slider.set_trigger(no_trigger);
        }
        self.add_bookmark_button.set_trigger(trigger);
        let menu_trigger = fltk::enums::CallbackTrigger::NotChanged
            | fltk::enums::CallbackTrigger::Release
            | fltk::enums::CallbackTrigger::ReleaseAlways;
        if has_history {
            self.history_menu_button.set_trigger(menu_trigger);
        } else {
            self.history_menu_button.set_trigger(no_trigger);
        }
        if has_bookmarks {
            self.bookmarks_menu_button.set_trigger(menu_trigger);
            self.delete_bookmark_button.set_trigger(button_trigger);
        } else {
            self.bookmarks_menu_button.set_trigger(no_trigger);
            self.delete_bookmark_button.set_trigger(no_trigger);
        }
    }
}
