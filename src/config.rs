// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{
    APPNAME, BOOKMARKS_SIZE, DEF_HISTORY_SIZE, MAX_HISTORY_SIZE,
    MIN_HISTORY_SIZE, SCALE_MAX, SCALE_MIN, WINDOW_HEIGHT_MIN,
    WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::{app, dialog};
use ini::Ini;
use std::{collections::VecDeque, env, path::PathBuf};

type History = VecDeque<PathBuf>;
type Bookmarks = Vec<PathBuf>;

#[derive(Clone, Debug)]
pub struct Config {
    pub window_x: i32,
    pub window_y: i32,
    pub window_height: i32,
    pub window_width: i32,
    pub window_scale: f32,
    pub volume: f64,
    pub pos: f64,
    pub track: PathBuf,
    pub filename: PathBuf,
    pub history_size: usize,
    pub history: History,
    pub bookmarks: Bookmarks,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            filename: get_config_filename(),
            ..Default::default()
        };
        if let Ok(ini) = Ini::load_from_file(&config.filename) {
            if let Some(properties) = ini.section(Some(WINDOW_SECTION)) {
                read_window_properties(properties, &mut config);
            }
            if let Some(properties) = ini.section(Some(TRACK_SECTION)) {
                read_track_properties(properties, &mut config);
            }
            if let Some(properties) = ini.section(Some(HISTORY_SECTION)) {
                read_history(properties, &mut config);
            }
            if let Some(properties) = ini.section(Some(BOOKMARK_SECTION)) {
                read_bookmarks(properties, &mut config);
            }
        }
        config
    }

    pub fn save(&self) {
        if self.filename.to_string_lossy() == "" {
            self.warning("failed to save configuration: no filename");
        } else {
            let mut ini = Ini::new();
            ini.with_section(Some(WINDOW_SECTION))
                .set(X_KEY, self.window_x.to_string())
                .set(Y_KEY, self.window_y.to_string())
                .set(WIDTH_KEY, self.window_width.to_string())
                .set(HEIGHT_KEY, self.window_height.to_string())
                .set(SCALE_KEY, app::screen_scale(0).to_string());
            ini.with_section(Some(TRACK_SECTION))
                .set(VOLUME_KEY, self.volume.to_string())
                .set(POS_KEY, self.pos.to_string())
                .set(TRACK_KEY, self.track.to_string_lossy());
            ini.with_section(Some(HISTORY_SECTION))
                .set(HISTORY_SIZE_KEY, self.history_size.to_string());
            self.save_history(&mut ini);
            self.save_bookmarks(&mut ini);
            match ini.write_to_file(&self.filename) {
                Ok(_) => {}
                Err(err) => self.warning(&format!(
                    "failed to save configuration: {err}"
                )),
            }
        }
    }

    fn save_history(&self, ini: &mut Ini) {
        for (i, track) in self.history.iter().enumerate() {
            let key = format!("{HISTORY_KEY}{}", i + 1);
            ini.with_section(Some(HISTORY_SECTION))
                .set(key, track.to_string_lossy());
        }
    }

    fn save_bookmarks(&self, ini: &mut Ini) {
        for (i, track) in self.bookmarks.iter().enumerate() {
            let key = format!("{BOOKMARK_KEY}{}", i + 1);
            ini.with_section(Some(BOOKMARK_SECTION))
                .set(key, track.to_string_lossy());
        }
    }

    fn warning(&self, message: &str) {
        dialog::message_title(&format!("Warning — {APPNAME}",));
        dialog::message(util::x() - 200, util::y() - 100, message);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_x: -1,
            window_y: -1,
            window_height: 60,
            window_width: 240,
            window_scale: 1.0,
            volume: 0.5,
            pos: 0.0,
            track: PathBuf::new(),
            filename: PathBuf::new(),
            history_size: DEF_HISTORY_SIZE,
            history: History::new(),
            bookmarks: Bookmarks::new(),
        }
    }
}

fn get_config_filename() -> PathBuf {
    let mut dir = dirs::config_dir();
    let mut dot = "";
    if dir.is_none() {
        if env::consts::FAMILY == "unix" {
            dot = ".";
        }
        dir = dirs::home_dir();
    }
    if let Some(dir) = dir {
        dir.join(format!("{dot}{}.ini", APPNAME.to_lowercase()))
    } else {
        PathBuf::new()
    }
}

fn read_window_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    let max_x = (app::screen_size().0 - 100.0) as i32;
    let max_y = (app::screen_size().1 - 100.0) as i32;
    if let Some(value) = properties.get(X_KEY) {
        config.window_x = util::get_num(value, 0, max_x, config.window_x)
    }
    if let Some(value) = properties.get(Y_KEY) {
        config.window_y = util::get_num(value, 0, max_y, config.window_y)
    }
    if let Some(value) = properties.get(WIDTH_KEY) {
        config.window_width = util::get_num(
            value,
            WINDOW_WIDTH_MIN,
            max_x,
            config.window_width,
        )
    }
    if let Some(value) = properties.get(HEIGHT_KEY) {
        config.window_height = util::get_num(
            value,
            WINDOW_HEIGHT_MIN,
            max_y,
            config.window_height,
        )
    }
    if let Some(value) = properties.get(SCALE_KEY) {
        config.window_scale =
            util::get_num(value, SCALE_MIN, SCALE_MAX, config.window_scale);
        if !util::isone32(config.window_scale) {
            app::set_screen_scale(0, config.window_scale);
        }
    }
}

fn read_track_properties(
    properties: &ini::Properties,
    config: &mut Config,
) {
    if let Some(value) = properties.get(VOLUME_KEY) {
        config.volume = util::get_num(value, 0.0, 1.0, config.volume)
    }
    if let Some(value) = properties.get(POS_KEY) {
        config.pos = util::get_num(value, 0.0, f64::MAX, config.pos)
    }
    if let Some(value) = properties.get(TRACK_KEY) {
        config.track = PathBuf::from(value);
    }
}

fn read_history(properties: &ini::Properties, config: &mut Config) {
    if let Some(value) = properties.get(HISTORY_SIZE_KEY) {
        config.history_size = util::get_num(
            value,
            MIN_HISTORY_SIZE,
            MAX_HISTORY_SIZE,
            config.history_size,
        )
    }
    config.history.clear();
    for i in 1..=config.history_size {
        let key = format!("{HISTORY_KEY}{i}");
        if let Some(value) = properties.get(&key) {
            let value = PathBuf::from(value);
            if !config.history.contains(&value) && value.exists() {
                config.history.push_back(value);
            }
        }
    }
}

fn read_bookmarks(properties: &ini::Properties, config: &mut Config) {
    config.bookmarks.clear();
    for i in 1..=BOOKMARKS_SIZE {
        let key = format!("{BOOKMARK_KEY}{i}");
        if let Some(value) = properties.get(&key) {
            let value = PathBuf::from(value);
            if !config.bookmarks.contains(&value) && value.exists() {
                config.bookmarks.push(value);
            }
        }
    }
}

static WINDOW_SECTION: &str = "Window";
static X_KEY: &str = "x";
static Y_KEY: &str = "y";
static WIDTH_KEY: &str = "width";
static HEIGHT_KEY: &str = "height";
static SCALE_KEY: &str = "scale";
static TRACK_SECTION: &str = "Track";
static VOLUME_KEY: &str = "volume";
static POS_KEY: &str = "pos";
static TRACK_KEY: &str = "track";
static HISTORY_SECTION: &str = "History";
static HISTORY_SIZE_KEY: &str = "size";
static HISTORY_KEY: &str = "history";
static BOOKMARK_SECTION: &str = "Boomarks";
static BOOKMARK_KEY: &str = "boomark";
