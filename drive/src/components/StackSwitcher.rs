use gtk::{Stack, StackSwitcher, glib::object::Cast};

pub struct drive_stack_switcher<'a> {
    width: i32,
    height: i32,
    global_margin: i32,
    stack: &'a mut Stack,
    wv: &'a mut Vec<gtk::Widget>,
}

pub fn driveStackSwitcher(
    w: i32,
    h: i32,
    global_margin: i32,
    stack: &mut Stack,
    widget_vector: &mut Vec<gtk::Widget>,
) -> StackSwitcher {
    let drive_switcher = drive_stack_switcher {
        width: w,
        height: h,
        global_margin: global_margin,
        stack: stack,
        wv: widget_vector,
    };

    let switcher = StackSwitcher::builder()
        .visible(true)
        .height_request(drive_switcher.height)
        .width_request(drive_switcher.width)
        .margin_bottom(drive_switcher.global_margin)
        .margin_top(drive_switcher.global_margin)
        .margin_start(drive_switcher.global_margin)
        .margin_end(drive_switcher.global_margin)
        .stack(drive_switcher.stack)
        .build();

    drive_switcher.wv.push(switcher.clone().upcast());

    return switcher;
}
