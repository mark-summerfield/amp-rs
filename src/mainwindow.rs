// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    Action, ABOUT_ICON, APPNAME, BUTTON_HEIGHT, HELP_ICON, ICON, LOAD_ICON,
    NEXT_ICON, OPTIONS_ICON, PAD, PLAY_ICON, PREV_ICON, QUIT_ICON,
    REPLAY_ICON, TIME_ICON, TOOLBAR_HEIGHT, TOOLBUTTON_SIZE, VOLUME_ICON,
};
use crate::util;
use fltk::prelude::*;

pub struct Widgets {
    pub mainwindow: fltk::window::Window,
    pub play_pause_button: fltk::button::Button,
    pub info_view: fltk::misc::HelpView,
    pub volume_slider: fltk::valuator::HorNiceSlider,
    pub volume_label: fltk::frame::Frame,
    pub time_slider: fltk::valuator::HorNiceSlider,
    pub time_label: fltk::frame::Frame,
}

pub fn make(sender: fltk::app::Sender<Action>) -> Widgets {
    fltk::window::Window::set_default_xclass(APPNAME);
    let icon = fltk::image::SvgImage::from_data(ICON).unwrap();
    let (x, y, width, height) = get_config_window_rect();
    let mut mainwindow =
        fltk::window::Window::new(x, y, width, height, APPNAME);
    mainwindow.set_icon(Some(icon));
    let size = ((TOOLBUTTON_SIZE * 4) / 3) * 6;
    mainwindow.size_range(size, size, size * 4, size * 4);
    mainwindow.make_resizable(true);
    let mut vbox = fltk::group::Flex::default()
        .size_of_parent()
        .with_type(fltk::group::FlexType::Column);
    vbox.set_margin(PAD);
    let mut info_view = fltk::misc::HelpView::default();
    info_view
        .set_value("<font color=green>Click Open to load a track…</font>");
    info_view.set_text_font(fltk::enums::Font::Helvetica);
    info_view.set_text_size((info_view.text_size() as f64 * 1.5) as i32);
    let (mut volume_box, volume_slider, volume_label) =
        add_volume_row(width);
    vbox.set_size(&volume_box, BUTTON_HEIGHT);
    let (mut time_box, time_slider, time_label) = add_time_row(width);
    vbox.set_size(&time_box, BUTTON_HEIGHT);
    let (play_pause_button, toolbar) = add_toolbar(sender, width);
    vbox.set_size(&toolbar, TOOLBAR_HEIGHT);
    vbox.end();
    mainwindow.end();
    Widgets {
        mainwindow,
        play_pause_button,
        info_view,
        volume_slider,
        volume_label,
        time_slider,
        time_label,
    }
}

fn add_toolbar(
    sender: fltk::app::Sender<Action>,
    width: i32,
) -> (fltk::button::Button, fltk::group::Flex) {
    let mut button_box = fltk::group::Flex::default()
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    button_box.set_frame(fltk::enums::FrameType::UpBox);
    button_box.set_margin(PAD);
    add_toolbutton(
        sender,
        'l',
        "Load Track • n",
        Action::Load,
        LOAD_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'b',
        "Back • b",
        Action::Previous,
        PREV_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'r',
        "Replay • r",
        Action::Replay,
        REPLAY_ICON,
        &mut button_box,
    );
    let play_pause_button = add_toolbutton(
        sender,
        'p',
        "Play/Pause • p or Space",
        Action::PlayOrPause,
        PLAY_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'n',
        "Next • n",
        Action::Next,
        NEXT_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'o',
        "Options… • o",
        Action::Options,
        OPTIONS_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'a',
        "About • a",
        Action::About,
        ABOUT_ICON,
        &mut button_box,
    );
    add_toolbutton(
        sender,
        'h',
        "Help • F1 or h",
        Action::Help,
        HELP_ICON,
        &mut button_box,
    );
    fltk::frame::Frame::default().with_size(PAD, PAD);
    add_toolbutton(
        sender,
        'q',
        "Quit • Esc or q",
        Action::Quit,
        QUIT_ICON,
        &mut button_box,
    );
    button_box.end();
    (play_pause_button, button_box)
}

fn add_toolbutton(
    sender: fltk::app::Sender<Action>,
    shortcut: char,
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
    button.set_shortcut(fltk::enums::Shortcut::from_char(shortcut));
    button.set_tooltip(tooltip);
    let mut icon = fltk::image::SvgImage::from_data(icon).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    button.set_image(Some(icon));
    button.emit(sender, action);
    button_box.set_size(&button, width);
    button
}

fn add_volume_row(
    width: i32,
) -> (fltk::group::Flex, fltk::valuator::HorNiceSlider, fltk::frame::Frame)
{
    let mut volume_box = fltk::group::Flex::default()
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    volume_box.set_margin(PAD / 2);
    let icon_height = TOOLBUTTON_SIZE + PAD;
    let icon_width = icon_height + 8;
    let mut volume_icon = fltk::frame::Frame::default();
    volume_icon.set_size(icon_width, icon_height);
    volume_icon.visible_focus(false);
    volume_icon.set_label_size(0);
    let mut icon = fltk::image::SvgImage::from_data(VOLUME_ICON).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    volume_icon.set_image(Some(icon));
    let mut volume_slider = fltk::valuator::HorNiceSlider::default();
    volume_slider.set_range(0.0, 1.0);
    volume_slider.set_step(1.0, 10); // 1/10
    {
        let config = CONFIG.get().read().unwrap();
        //volume_slider.set_value(config.volume); / TODO
    }
    let mut volume_label = fltk::frame::Frame::default().with_label("0%");
    volume_label.set_frame(fltk::enums::FrameType::EngravedFrame);
    volume_box.set_size(&volume_icon, icon_width);
    volume_box.set_size(&volume_label, icon_width * 2);
    volume_box.end();
    (volume_box, volume_slider, volume_label)
}

fn add_time_row(
    width: i32,
) -> (fltk::group::Flex, fltk::valuator::HorNiceSlider, fltk::frame::Frame)
{
    let mut time_box = fltk::group::Flex::default()
        .with_size(width, TOOLBAR_HEIGHT)
        .with_type(fltk::group::FlexType::Row);
    time_box.set_margin(PAD / 2);
    let icon_height = TOOLBUTTON_SIZE + PAD;
    let icon_width = icon_height + 8;
    let mut time_icon = fltk::frame::Frame::default();
    time_icon.set_size(icon_width, icon_height);
    time_icon.visible_focus(false);
    time_icon.set_label_size(0);
    let mut icon = fltk::image::SvgImage::from_data(TIME_ICON).unwrap();
    icon.scale(TOOLBUTTON_SIZE, TOOLBUTTON_SIZE, true, true);
    time_icon.set_image(Some(icon));
    let mut time_slider = fltk::valuator::HorNiceSlider::default();
    let mut time_label = fltk::frame::Frame::default().with_label("0″/0″");
    time_label.set_frame(fltk::enums::FrameType::EmbossedFrame);
    time_box.set_size(&time_icon, icon_width);
    time_box.set_size(&time_label, icon_width * 2);
    time_box.end();
    (time_box, time_slider, time_label)
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
    mainwindow: &mut fltk::window::Window,
    sender: fltk::app::Sender<Action>,
) {
    // Both of these are really needed!
    mainwindow.set_callback(move |_| {
        if fltk::app::event() == fltk::enums::Event::Close
            || fltk::app::event_key() == fltk::enums::Key::Escape
        {
            sender.send(Action::Quit);
        }
    });
    mainwindow.handle(move |_, event| match event {
        fltk::enums::Event::KeyUp => match fltk::app::event_key() {
            fltk::enums::Key::Help | fltk::enums::Key::F1 => {
                sender.send(Action::Help);
                true
            }
            _ => false,
        },
        _ => false,
    });
}
