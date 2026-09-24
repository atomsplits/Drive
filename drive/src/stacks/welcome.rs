use gtk::{FilterMatch::Some, Label, Stack};

#[warn(dead_code)]
#[allow(unused)]

pub fn welcome_stack() -> Stack {
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideRight); // customize this later

    let wlc = Label::builder().label("Welcome to Drive");

    return stack;
}
