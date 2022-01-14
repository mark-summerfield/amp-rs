// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use std::env;

pub static APPNAME: &str = "AMP";
pub static VERSION: &str = "1.2.2";
pub static HELP_HTML: &str = include_str!("../data/help.html");
pub const ICON: &str = include_str!("../images/amp.svg");
pub const LOAD_ICON: &str = include_str!("../images/document-open.svg");
pub const PREV_ICON: &str =
    include_str!("../images/media-seek-backward.svg");
pub const REPLAY_ICON: &str = include_str!("../images/replay.svg");
pub const PLAY_ICON: &str =
    include_str!("../images/media-playback-start.svg");
pub const PAUSE_ICON: &str =
    include_str!("../images/media-playback-pause.svg");
pub const NEXT_ICON: &str =
    include_str!("../images/media-seek-forward.svg");
pub const VOLUME_ICON: &str =
    include_str!("../images/audio-volume-high.svg");
pub const TIME_ICON: &str = include_str!("../images/time.svg");
pub const HISTORY_ICON: &str = include_str!("../images/history.svg");
pub const BOOKMARKS_ICON: &str = include_str!("../images/bookmarks.svg");
pub const ADD_BOOKMARK_ICON: &str =
    include_str!("../images/addbookmark.svg");
pub const DELETE_BOOKMARK_ICON: &str =
    include_str!("../images/deletebookmark.svg");
pub const MENU_ICON: &str = include_str!("../images/menu.svg");
pub const DEF_HISTORY_SIZE: usize = 26;
pub const MIN_HISTORY_SIZE: usize = 2;
pub const MAX_HISTORY_SIZE: usize = 35;
pub const BOOKMARKS_SIZE: usize = 35;
pub const PAD: i32 = 6;
pub const WINDOW_WIDTH_MIN: i32 = 480;
pub const WINDOW_HEIGHT_MIN: i32 = 240;
pub const TOOLBUTTON_SIZE: i32 = 28;
pub const TOOLBAR_HEIGHT: i32 = ((TOOLBUTTON_SIZE * 3) / 2) + (2 * PAD);
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const TINY_TIMEOUT: f64 = 0.075;
pub const TICK_TIMEOUT: f64 = 0.1;
pub static MENU_DIGITS: [char; 9] =
    ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub static MENU_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const PATH_SEP: char = '→';

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    About,
    AddBookmark,
    AddToHistory,
    DeleteBookmark,
    Help,
    Load,
    LoadBookmarkedTrack,
    LoadHistoryTrack,
    MainMenu,
    Next,
    OnBookmarkMenu,
    OnHistoryMenu,
    OnMenuMenu,
    OnStartup,
    Options,
    PlayOrPause,
    Previous,
    Quit,
    Replay,
    SpacePressed,
    Tick,
    TimeUpdate,
    VolumeDown,
    VolumeUp,
    VolumeUpdate,
}

pub fn about_html(player: &soloud::Soloud) -> String {
    let year = Local::today().year();
    let year = if year == 2021 {
        year.to_string()
    } else {
        format!("2021-{}", year - 2000)
    };
    format!(
        "<p><center><font size=6 color=navy><b>{}</b> v{}</font>
</center></p>
<p><center><font color=navy size=5>“Another Music Player‟</font>
</center></p>
<p><center><font size=4>
<a href=\"https://github.com/mark-summerfield/amp-rs\">https://github.com/mark-summerfield/amp-rs</a>
</font></center></p>
<p><center>
<font size=4 color=green>
Copyright © {} Mark Summerfield.<br>
All rights reserved.<br>
License: GPLv3.</font>
</center></p>
<p><center><font size=4 color=#555>
Rust {} • fltk-rs {} • FLTK {}<br>Soloud {}/{} • {}/{}
</font></center></p>",
        APPNAME,
        VERSION,
        year,
        rustc_version_runtime::version(),
        fltk::app::crate_version(),
        fltk::app::version_str(),
        player.version(),
        player.backend_string(),
        capitalize_first(env::consts::OS),
        env::consts::ARCH
    )
}

pub static LOAD_ERROR: &str = "
<font color=red><b>Error</b><br>Failed to open</font>
<font color=magenta>\"FILE\".</font>
</body>";
