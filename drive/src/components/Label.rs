use gtk::{Align, Justification, Label, pango::WrapMode};

pub struct DriveLabel<'a> {
    text: String,
    justify: Option<Justification>,
    xalign: Option<f32>,
    yalign: Option<f32>,
    halign: Option<Align>,
    valign: Option<Align>,
    global_margin: i32,
    wrap_mode: WrapMode,
    widget_vector: &'a mut Vec<gtk::Widget>,
}
