use gtk::{Align, Button, glib::object::Cast};
#[warn(dead_code)]
#[allow(unused)] // i genuinely hate this warning lol

pub struct DriveButton<'a> {
    content: String,
    width: i32,
    height: i32,
    global_margin: i32,
    vertical_alignment: Align,
    horizontal_alignment: Align,
    widget_vector: &'a mut Vec<gtk::Widget>,
}

pub fn drive_btn(
    content: String,
    width: i32,
    height: i32,
    global_margin: i32,
    va: Align,
    ha: Align,
    wv: &mut Vec<gtk::Widget>,
) -> Button {
    let btn = DriveButton {
        content: content,
        width: width,
        height: height,
        global_margin: global_margin,
        vertical_alignment: va,
        horizontal_alignment: ha,
        widget_vector: wv,
    };

    let gtk_btn = Button::builder()
        .label(btn.content)
        .margin_bottom(global_margin)
        .margin_top(global_margin)
        .margin_start(global_margin)
        .margin_end(global_margin)
        .height_request(height)
        .width_request(width)
        .halign(ha)
        .valign(va)
        .build();

    wv.push(gtk_btn.clone().upcast());

    return gtk_btn;
}
