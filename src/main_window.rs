// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, ADD_BOOKMARK_ICON, APPNAME, A_TO_Z, BOOKMARKS_ICON,
    BUTTON_HEIGHT, DELETE_BOOKMARK_ICON, HISTORY_ICON, ICON, LOAD_ICON,
    MENU_ICON, NEXT_ICON, PAD, PATH_SEP, PLAY_ICON, PREV_ICON, REPLAY_ICON,
    TIME_ICON, TOOLBAR_HEIGHT, TOOLBUTTON_SIZE, VOLUME_ICON,
    WINDOW_HEIGHT_MIN, WINDOW_WIDTH_MIN,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub main_window: fltk::window::Window,
    pub play_pause_button: fltk::button::Button,
    pub history_menu_button: fltk::menu::MenuButton,
    pub bookmarks_menu_button: fltk::menu::MenuButton,
    pub info_view: fltk::misc::HelpView,
    pub volume_slider: fltk::valuator::HorFillSlider,
    pub volume_label: fltk::frame::Frame,
    pub time_slider: fltk::valuator::HorFillSlider,
    pub time_label: fltk::frame::Frame,
}

pub fn make(sender: fltk::app::Sender<Action>) -> Widgets {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut main_window =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    main_window.set_icon(Some(icon));
    main_window.size_range(
        WINDOW_WIDTH_MIN,
        WINDOW_HEIGHT_MIN,
        fltk::app::screen_size().0 as i32,
        fltk::app::screen_size().1 as i32,
    );
    main_window.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    let info_view = add_info_view();
    let (volume_box, volume_slider, volume_label) = add_volume_row(width);
    vbox.set_size(&volume_box, BUTTON_HEIGHT);
    let (time_box, time_slider, time_label) =
        add_slider_row(width, TIME_ICON, "0″/0″");
    vbox.set_size(&time_box, BUTTON_HEIGHT);
    let (
        play_pause_button,
        history_menu_button,
        bookmarks_menu_button,
        toolbar,
    ) = add_toolbar(sender, width);
    vbox.set_size(&toolbar, TOOLBAR_HEIGHT);
    vbox.end();
    main_window.end();
    Widgets {
        main_window,
        play_pause_button,
        history_menu_button,
        bookmarks_menu_button,
        info_view,
        volume_slider,
        volume_label,
        time_slider,
        time_label,
    }
}

fn add_info_view() -> fltk::misc::HelpView {
    let mut info_view = fltk::misc::HelpView::default();
    info_view
        .set_value("<font color=green>Click Open to load a track…</font>");
    info_view.set_text_font(fltk::enums::Font::Helvetica);
    info_view.set_text_size((info_view.text_size() as f64 * 1.3) as i32);
    info_view
}

fn add_toolbar(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (
    fltk::button::Button,
    fltk::menu::MenuButton,
    fltk::menu::MenuButton,
    fltk::group::Flex,
) {
    let mut button_box = fltk::group::Flex::default()
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    button_box.set_frame(fltk::enums::FrameType::UpBox);
    button_box.set_margin(PAD);
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_char('o'),
        "Open a track ready to play • o",
        Action::Load,
        LOAD_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_key(fltk::enums::Key::F4),
        "Previous track • F4",
        Action::Previous,
        PREV_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_char('r'),
        "Replay the current track • r or F5",
        Action::Replay,
        REPLAY_ICON,
        &mut button_box,
    );
    let play_pause_button = add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_char('p'),
        "Play or Pause the current track • p or Space",
        Action::PlayOrPause,
        PLAY_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_key(fltk::enums::Key::F6),
        "Next track • F6",
        Action::Next,
        NEXT_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    let mut history_menu_button =
        add_menubutton(0x68, "History • h", HISTORY_ICON, &mut button_box);
    populate_history_menu_button(&mut history_menu_button, sender);
    let mut bookmarks_menu_button = add_menubutton(
        0x62,
        "Bookmarks • b",
        BOOKMARKS_ICON,
        &mut button_box,
    );
    populate_bookmarks_menu_button(&mut bookmarks_menu_button, sender);
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_char('a'),
        "Add Track to Bookmarks • a",
        Action::AddBookmark,
        ADD_BOOKMARK_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        fltk::enums::Shortcut::from_char('d'),
        "Delete Track from Bookmarks • d",
        Action::DeleteBookmark,
        DELETE_BOOKMARK_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    let mut menu_button =
        add_menubutton(0x6D, "Menu • m", MENU_ICON, &mut button_box);
    initialize_menu_button(&mut menu_button, sender);
    button_box.end();
    (
        play_pause_button,
        history_menu_button,
        bookmarks_menu_button,
        button_box,
    )
}

fn add_toolbutton(
    sender: fltk::app::Sender<Action>,
    shortcut: fltk::enums::Shortcut,
    tooltip: &str,
    action: Action,
    icon: &str,
    button_box: &mut fltk::group::Flex,
) -> fltk::button::Button {
    let width = TOOLBUTTON_SIZE + PAD + 8;
    let mut button = fltk::button::Button::default();
    button.set_size(width, TOOLBUTTON_SIZE + PAD);
    button.visible_focus(false);
    button.set_label_size(0);
    button.set_shortcut(shortcut);
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::SvgImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.emit(sender, action);
    button_box.set_size(&button, width);
    button
}

fn add_menubutton(
    codepoint: i32,
    tooltip: &str,
    icon: &str,
    button_box: &mut fltk::group::Flex,
) -> fltk::menu::MenuButton {
    let width = TOOLBUTTON_SIZE + PAD + 8;
    let mut button = fltk::menu::MenuButton::default();
    button.set_size(width, TOOLBUTTON_SIZE + PAD);
    button.visible_focus(false);
    button.set_label_size(0);
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::SvgImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.handle(move |button, event| {
        if event == fltk::enums::Event::KeyUp
            && fltk::app::event_key().bits() == codepoint
        {
            button.popup();
            return true;
        }
        false
    });
    button_box.set_size(&button, width);
    button
}

pub(crate) fn populate_history_menu_button(
    menu_button: &mut fltk::menu::MenuButton,
    sender: fltk::app::Sender<Action>,
) {
    menu_button.clear();
    let config = CONFIG.get().read().unwrap();
    for (i, track) in config.history.iter().enumerate() {
        menu_button.add_emit(
            &track_menu_option(i, track),
            fltk::enums::Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            sender,
            Action::LoadHistoryTrack,
        );
    }
}

pub(crate) fn populate_bookmarks_menu_button(
    menu_button: &mut fltk::menu::MenuButton,
    sender: fltk::app::Sender<Action>,
) {
    menu_button.clear();
    let config = CONFIG.get().read().unwrap();
    for (i, track) in config.bookmarks.iter().enumerate() {
        menu_button.add_emit(
            &track_menu_option(i, track),
            fltk::enums::Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            sender,
            Action::LoadBookmarkedTrack,
        );
    }
}

fn track_menu_option(index: usize, track: &std::path::PathBuf) -> String {
    format!(
        "&{} {}",
        A_TO_Z[index],
        track
            .to_string_lossy()
            .replace(&['\\', '/'][..], &PATH_SEP.to_string())
    )
}

fn initialize_menu_button(
    menu_button: &mut fltk::menu::MenuButton,
    sender: fltk::app::Sender<Action>,
) {
    menu_button.set_label("&Menu");
    menu_button.add_emit(
        "&Options…",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::MenuDivider,
        sender,
        Action::Options,
    );
    menu_button.add_emit(
        "&Help • F1",
        fltk::enums::Shortcut::None, // handled elsewhere
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::Help,
    );
    menu_button.add_emit(
        "&About",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::MenuDivider,
        sender,
        Action::About,
    );
    menu_button.add_emit(
        "&Quit • Esc",
        fltk::enums::Shortcut::None, // handled elsewhere
        fltk::menu::MenuFlag::Normal,
        sender,
        Action::Quit,
    );
}

fn add_volume_row(
    width: i32,
) -> (fltk::group::Flex, fltk::valuator::HorFillSlider, fltk::frame::Frame)
{
    let (volume_box, mut volume_slider, volume_label) =
        add_slider_row(width, VOLUME_ICON, "0%");
    volume_slider.set_range(0.0, 1.0);
    volume_slider.set_step(1.0, 10); // 1/10
    (volume_box, volume_slider, volume_label)
}

fn add_slider_row(
    width: i32,
    icon: &str,
    label: &str,
) -> (fltk::group::Flex, fltk::valuator::HorFillSlider, fltk::frame::Frame)
{
    let mut row = fltk::group::Flex::default()
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    row.set_margin(PAD / 2);
    let icon_height = TOOLBUTTON_SIZE + PAD;
    let icon_width = icon_height + 8;
    let mut icon_label = fltk::frame::Frame::default();
    icon_label.set_size(icon_width, icon_height);
    icon_label.visible_focus(false);
    icon_label.set_label_size(0);
    let mut icon_image = fltk::image::SvgImage::from_data(icon).unwrap();
    icon_image.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    icon_label.set_image(Some(icon_image));
    let slider = fltk::valuator::HorFillSlider::default();
    let mut label = fltk::frame::Frame::default().with_label(label);
    label.set_frame(fltk::enums::FrameType::EngravedFrame);
    row.set_size(&icon_label, icon_width);
    row.set_size(&label, icon_width * 3);
    row.end();
    (row, slider, label)
}

fn get_config_window_rect() -> (i32, i32, i32, i32) {
    let mut config = CONFIG.get().write().unwrap();
    let x = if config.window_x >= 0 {
        config.window_x
    } else {
        util::x() - (config.window_width / 2)
    };
    let y = if config.window_y >= 0 {
        config.window_y
    } else {
        util::y() - (config.window_height / 2)
    };
    if x != config.window_x {
        config.window_x = x;
    }
    if y != config.window_y {
        config.window_y = y;
    }
    (x, y, config.window_width, config.window_height)
}

pub fn add_event_handlers(
    main_window: &mut fltk::window::Window,
    sender: fltk::app::Sender<Action>,
) {
    // Both of these are really needed!
    main_window.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    main_window.handle(move |_, event| {
        if event == fltk::enums::Event::KeyUp {
            let key = fltk::app::event_key();
            if key.bits() == 0x20 {
                sender.send(Action::SpacePressed); // Space → Play or Pause
                return true;
            }
            if key.bits() == 0x2B || key.bits() == 0x3D {
                sender.send(Action::VolumeUp); // + or =
                return true;
            }
            if key.bits() == 0x2D {
                sender.send(Action::VolumeDown); // -
                return true;
            }
            if fltk::app::event_key() == fltk::enums::Key::Help
                || fltk::app::event_key() == fltk::enums::Key::F1
            {
                sender.send(Action::Help);
                return true;
            }
            if fltk::app::event_key() == fltk::enums::Key::F5 {
                sender.send(Action::Replay);
                return true;
            }
        }
        false
    });
}

pub fn update_widgets_from_config(widgets: &mut Widgets) -> bool {
    let config = CONFIG.get().read().unwrap();
    widgets.volume_slider.set_value(config.volume);
    widgets
        .volume_label
        .set_label(&format!("{}%", (config.volume * 100.0).round()));
    config.track.exists()
}
