use gtk4::{
    Application, ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
    prelude::GtkWindowExt,
};

fn main() -> glib::ExitCode {
    let drive = Application::builder()
        .application_id("com.roundhouses.Drive")
        .build();

    drive.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(400)
            .title("Drive")
            .build();

        window.present();
    });

    drive.run()
}
