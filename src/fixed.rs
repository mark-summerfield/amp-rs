// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use std::env;

pub static APPNAME: &str = "AMP";
pub static VERSION: &str = "0.1.0";
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
pub const OPTIONS_ICON: &str = include_str!("../images/options.svg");
pub const ABOUT_ICON: &str = include_str!("../images/about.svg");
pub const HELP_ICON: &str = include_str!("../images/help.svg");
pub const QUIT_ICON: &str = include_str!("../images/quit.svg");
pub const PAD: i32 = 6;
pub const WINDOW_WIDTH_MIN: i32 = 440;
pub const WINDOW_HEIGHT_MIN: i32 = 160;
pub const TOOLBUTTON_SIZE: i32 = 28;
pub const TOOLBAR_HEIGHT: i32 = ((TOOLBUTTON_SIZE * 3) / 2) + (2 * PAD);
pub const BUTTON_HEIGHT: i32 = 30;
pub const BUTTON_WIDTH: i32 = 70;
pub const SCALE_MIN: f32 = 0.5;
pub const SCALE_MAX: f32 = 3.5;
pub const TINY_TIMEOUT: f64 = 0.075;
pub const TICK_TIMEOUT: f64 = 0.1;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Load,
    Previous,
    Replay,
    PlayOrPause,
    SpacePressed,
    Tick,
    Next,
    VolumeDown,
    VolumeUp,
    VolumeUpdate,
    Options,
    About,
    Help,
    Quit,
}

pub fn about_html(player: &soloud::Soloud) -> String {
    let year = Local::today().year();
    let year = if year == 2021 {
        format!("{}", year)
    } else {
        format!("2021-{}", year - 2000)
    };
    format!(
        "<p><center><font size=6 color=navy><b>{}</b> v{}</font>
</center></p>
<p><center><font color=navy size=5>“Another Music Player‟</font>
</center></p>
<p><center><font size=4>
<a href=\"http://www.qtrac.eu/amp.html\">www.qtrac.eu/amp.html</a>
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

// TODO update table with keyboard shortcuts
pub static HELP_HTML: &str = "<body>
<p><center><font color=navy size=6><b>AMP</b></font></center></p>
<p><center><font color=blue size=5>“Another Music Player‟</font>
</center></p>
<p>
<table border=1 align=center>
<font color=blue>
<tr><th>Key</th><th>Action</th></tr>
<tr><td><b>a</b></td><td>Show About box</td></tr>
<tr><td><b>h</b> or <b>F1</b></td><td>Show this Help window</td></tr>
<tr><td><b>n</b></td><td>New Game</td></tr>
<tr><td><b>o</b></td><td>View or Edit Options</td></tr>
<tr><td><b>q</b> or <b>Esc</b></td><td>Quit</td></tr>
<tr><td><b>←</b></td><td>Move the focus left</td></tr>
<tr><td><b>→</b></td><td>Move the focus right</td></tr>
<tr><td><b>↑</b></td><td>Move the focus up</td></tr>
<tr><td><b>↓</b></td><td>Move the focus down</td></tr>
<tr><td><b>Space</b></td><td>Click the focused tile</td></tr>
</font>
</table>
</body>";
