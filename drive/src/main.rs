use gtk::{
    Align::Center,
    Application, ApplicationWindow, Widget,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::{self},
    prelude::{ButtonExt, GtkWindowExt},
};

use crate::components::{Button::drive_btn, StackSwitcher::driveStackSwitcher};
mod components;
mod stacks;

const DRIVE_ID: &str = "com.atomsplits.Drive";

fn main() -> glib::ExitCode {
    let drive = Application::builder().application_id(DRIVE_ID).build();

    drive.connect_activate(build_ui);

    drive.run()
}

fn build_ui(app: &Application) {
    // widget vec
    let mut widgets: Vec<Widget> = Vec::new();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Drive")
        .build();

    let button = drive_btn(
        "Play".to_string(),
        100,
        75,
        20,
        Center,
        Center,
        &mut widgets,
    );

    button.connect_clicked(|button| {
        button.set_label("Now Playing:");
    });

    let settings_switcher = driveStackSwitcher(
        100,
        75,
        20,
        &mut stacks::welcome::welcome_stack(),
        &mut widgets,
    );
    //settings_switcher.set_stack(Some(&stacks::welcome::welcome_stack()));

    // add widgets to window
    for widget in widgets.iter_mut() {
        window.set_child(Some(widget));
    }

    window.present();
}
