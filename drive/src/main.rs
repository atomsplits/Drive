use gtk::{
    Align::Center,
    Application, ApplicationWindow, Button,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
    prelude::{ButtonExt, GtkWindowExt},
};

use crate::components::Button::drive_btn;
mod components;

const DRIVE_ID: &str = "com.atomsplits.Drive";

fn main() -> glib::ExitCode {
    let drive = Application::builder().application_id(DRIVE_ID).build();

    drive.connect_activate(build_ui);

    drive.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Drive")
        .build();

    let button = drive_btn("Play".to_string(), 100, 75, 20);

    let g = button.connect_clicked(|button| {
        button.set_label("Now Playing:");
    });

    window.set_child(Some(&button));

    window.present();
}
