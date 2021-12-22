// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod config;
mod fixed;
mod html_form;
mod mainwindow;
mod options_form;
mod util;

use crate::application::Application;
use crate::fixed::APPNAME;
use std::{panic, sync};

pub static CONFIG: state::Storage<sync::RwLock<config::Config>> =
    state::Storage::new();

fn main() {
    panic::set_hook(Box::new(|info| {
        let err = dbg!(&info);
        fltk::dialog::message_title(&format!("Error — {}", APPNAME));
        let x = util::x() - 200;
        let y = util::y() - 100;
        fltk::dialog::message(x, y, &err.to_string());
    }));
    CONFIG.set(sync::RwLock::new(config::Config::new()));
    let mut app = Application::new();
    app.run();
}
