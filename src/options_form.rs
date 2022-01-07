// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use super::CONFIG;
use crate::fixed::{
    APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON, PAD, SCALE_MAX, SCALE_MIN,
};
use crate::util;
use fltk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Form {
    form: fltk::window::Window,
    pub ok: Rc<RefCell<bool>>,
}

impl Form {
    pub fn default() -> Self {
        let ok = Rc::from(RefCell::from(false));
        let mut form = make_form();
        let mut vbox =
            fltk::group::Flex::default().size_of_parent().column();
        vbox.set_margin(PAD);
        vbox.set_pad(PAD);
        make_config_row();
        let mut spinners = make_spinners();
        let (button_row, mut buttons) = make_buttons();
        vbox.set_size(&button_row, BUTTON_HEIGHT);
        vbox.end();
        form.end();
        form.make_modal(true);
        add_event_handlers(
            &mut form,
            &spinners,
            &mut buttons,
            Rc::clone(&ok),
        );
        spinners.scale_spinner.take_focus().unwrap();
        form.show();
        while form.shown() {
            fltk::app::wait();
        }
        Self { form, ok }
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        fltk::app::delete_widget(self.form.clone());
    }
}

struct Spinners {
    pub scale_spinner: fltk::misc::Spinner,
}

struct Buttons {
    pub ok_button: fltk::button::Button,
    pub cancel_button: fltk::button::Button,
}

fn make_form() -> fltk::window::Window {
    let image = fltk::image::SvgImage::from_data(ICON).unwrap();
    let mut form = fltk::window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(&format!("Configure — {}", APPNAME));
    if let Some(window) = fltk::app::first_window() {
        form.set_pos(window.x() + 50, window.y() + 100);
    }
    form.set_icon(Some(image));
    form
}

fn make_config_row() {
    let mut row = fltk::group::Flex::default().row();
    let label = fltk::frame::Frame::default()
        .with_label("Config")
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Left);
    let config = CONFIG.get().read().unwrap();
    let mut filename_label = fltk::frame::Frame::default()
        .with_label(&config.filename.to_string_lossy())
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Left);
    filename_label.set_frame(fltk::enums::FrameType::EngravedFrame);
    row.set_size(&label, WIDTH / 6);
    row.end();
}

fn make_spinners() -> Spinners {
    let config = CONFIG.get().read().unwrap();
    let scale_spinner = make_row(
        "&Scale",
        config.window_scale as f64,
        "User interface scale (default 1.0)",
        SCALE_MIN as f64,
        SCALE_MAX as f64,
        0.1,
    );
    Spinners { scale_spinner }
}

fn make_row(
    label: &str,
    value: f64,
    tooltip: &str,
    minimum: f64,
    maximum: f64,
    step: f64,
) -> fltk::misc::Spinner {
    let mut row = fltk::group::Flex::default().row();
    row.set_pad(PAD);
    let mut label = fltk::button::Button::default()
        .with_label(label)
        .with_align(fltk::enums::Align::Inside | fltk::enums::Align::Left);
    label.set_frame(fltk::enums::FrameType::NoBox);
    label.clear_visible_focus();
    let mut spinner = fltk::misc::Spinner::default();
    spinner.set_value(value);
    spinner.set_step(step);
    spinner.set_range(minimum, maximum);
    spinner.set_tooltip(tooltip);
    row.end();
    label.set_callback({
        let mut spinner = spinner.clone();
        move |_| {
            spinner.take_focus().unwrap();
        }
    });
    spinner
}

fn make_buttons() -> (fltk::group::Flex, Buttons) {
    let mut row = fltk::group::Flex::default().size_of_parent().row();
    row.set_pad(PAD);
    fltk::frame::Frame::default(); // pad left of buttons
    let ok_button = fltk::button::Button::default().with_label("&OK");
    let cancel_button =
        fltk::button::Button::default().with_label("&Cancel");
    fltk::frame::Frame::default(); // pad right of buttons
    row.set_size(&ok_button, BUTTON_WIDTH);
    row.set_size(&cancel_button, BUTTON_WIDTH);
    row.end();
    (row, Buttons { ok_button, cancel_button })
}

fn add_event_handlers(
    form: &mut fltk::window::Window,
    spinners: &Spinners,
    buttons: &mut Buttons,
    ok: Rc<RefCell<bool>>,
) {
    buttons.ok_button.set_callback({
        let scale_spinner = spinners.scale_spinner.clone();
        let mut form = form.clone();
        move |_| {
            *ok.borrow_mut() = true;
            let mut config = CONFIG.get().write().unwrap();
            let old_scale = config.window_scale;
            let new_scale = scale_spinner.value() as f32;
            if !util::isclose32(old_scale, new_scale) {
                config.window_scale = new_scale;
                fltk::app::set_screen_scale(0, new_scale);
            }
            form.hide();
        }
    });
    buttons.cancel_button.set_callback({
        let mut form = form.clone();
        move |_| {
            form.hide();
        }
    });
}

const WIDTH: i32 = 340;
const HEIGHT: i32 = 120;
