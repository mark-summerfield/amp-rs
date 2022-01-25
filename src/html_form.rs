// Copyright © 2021-22 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::fixed::{APPNAME, BUTTON_HEIGHT, BUTTON_WIDTH, ICON};
use fltk::{
    app, button::Button, enums::Font, frame::Frame, group::Flex,
    image::SvgImage, misc::HelpView, prelude::*, window::Window,
};

pub struct Form {
    form: Window,
}

impl Form {
    pub fn new(
        title: &str,
        html_text: &str,
        modal: bool,
        width: i32,
        height: i32,
        resizable: bool,
    ) -> Self {
        let (mut form, mut ok_button) =
            make_widgets(title, html_text, width, height, resizable);
        form.make_modal(modal);
        add_event_handler(&mut form, &mut ok_button);
        form.show();
        if modal {
            while form.shown() {
                app::wait();
            }
        }
        Self { form }
    }

    pub fn show(&mut self) {
        self.form.show();
    }
}

impl Drop for Form {
    fn drop(&mut self) {
        app::delete_widget(self.form.clone());
    }
}

fn make_widgets(
    title: &str,
    html_text: &str,
    width: i32,
    height: i32,
    resizable: bool,
) -> (Window, Button) {
    let image = SvgImage::from_data(ICON).unwrap();
    let mut form = Window::new(0, 0, width, height, "");
    if let Some(window) = app::first_window() {
        form.set_pos(window.x() + 50, window.y() + 100);
    }
    form.set_label(&format!("{title} — {APPNAME}"));
    form.make_resizable(resizable);
    form.set_icon(Some(image));
    let mut vbox = Flex::default().size_of_parent().column();
    let mut view = HelpView::default();
    view.set_value(html_text);
    view.set_text_font(Font::Helvetica);
    view.set_text_size((view.text_size() as f64 * 1.2) as i32);
    let mut button_row = Flex::default().size_of_parent().row();
    Frame::default(); // pad left of button
    let ok_button = Button::default().with_label("&OK");
    Frame::default(); // pad right of button
    button_row.set_size(&ok_button, BUTTON_WIDTH);
    button_row.end();
    vbox.set_size(&button_row, BUTTON_HEIGHT);
    vbox.end();
    form.end();
    (form, ok_button)
}

fn add_event_handler(form: &mut Window, ok_button: &mut Button) {
    ok_button.set_callback({
        let mut form = form.clone();
        move |_| {
            form.hide();
        }
    });
}
