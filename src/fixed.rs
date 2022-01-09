// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::capitalize_first;
use chrono::prelude::*;
use std::env;

pub static APPNAME: &str = "AMP";
pub static VERSION: &str = "1.0.4";
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
    TimeUpdate,
    Options,
    About,
    Help,
    Quit,
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

pub static HELP_HTML: &str = "<body>
<p><center><font color=navy size=6><b>AMP</b></font></center></p>
<p><center><font color=blue size=5>“Another Music Player‟</font>
</center></p>
<font color=blue>
<p>
Another Music Player provides a basic but useful example of Rust/FLTK and
the Soloud sound library.
</p>
<p>
It should be especially handy for audio books since at startup it restores
the last played track at the exact position it had reached. And when the end
of a track is reached, the next track in the same folder is automatically
played (if there is one).
</p>
<p>
Click the volume slider to change the volume (or press the <b>+</b> or
<b>-</b> keys). Similarly, click the time slider to change the position in the currently playing track.
</p>
</font>
<p>
<table border=1 align=center>
<font color=green>
<tr><th>Key</th><th>Action</th></tr>
<tr><td><b>-</b></td><td>Reduce the volume</td></tr>
<tr><td><b>+</b> or <b>=</b></td><td>Increase the volume</td></tr>
<tr><td><b>a</b></td><td>Show the About box</td></tr>
<tr><td><b>b</b></td><td>Go <i>B</i>ack to the previous track</td></tr>
<tr><td><b>c</b></td><td>Configure…</td></tr>
<tr><td><b>h</b> or <b>F1</b></td><td>Show this Help window</td></tr>
<tr><td><b>n</b></td><td>Go to the <i>N</i>ext track</td></tr>
<tr><td><b>o</b></td><td>Open a track ready to play</td></tr>
<tr><td><b>p</b> or <b>Space</b></td><td>Play or Pause the current
track</td></tr>
<tr><td><b>q</b> or <b>Esc</b></td><td>Quit</td></tr>
</font>
</table>
</body>";
