use gtk::{Button, prelude::ButtonExt};

pub struct DriveButton {
    content: String,
    width: i32,
    height: i32,
    global_margin: i32,
}

pub fn drive_btn(content: String, width: i32, height: i32, global_margin: i32) -> Button {
    let btn = DriveButton {
        content: content,
        width: width,
        height: height,
        global_margin: global_margin,
    };

    let gtk_btn = Button::builder()
        .label(btn.content)
        .margin_bottom(global_margin)
        .margin_top(global_margin)
        .margin_start(global_margin)
        .margin_end(global_margin)
        .height_request(height)
        .width_request(width)
        .build();

    return gtk_btn;
}
