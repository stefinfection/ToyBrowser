/* Controls drawing window and rendering graphical pieces within.
 *
 * Ref: https://github.com/gtk-rs/examples/blob/master/src/bin/css.rs
 */
extern crate gtk;
extern crate gio;
extern crate glib;
extern crate gdk;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

const STYLE: &str =
"#entry1 {
    background-image: -gtk-gradient (linear,
                                     0 0, 1 0,
                                     color-stop(0, #f00),
                                     color-stop(1, #0f0));
    color: blue;
    font-weight: bold;
}
button {
    /* If we don't put it, the yellow background won't be visible */
    background-image: none;
}
#label1:hover {
    transition: 500ms;
    color: red;
    background-color: yellow;
}
combobox button.combo box {
    padding: 5px;
}
combobox box arrow {
    -gtk-icon-source: none;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid black;
}";

pub fn init() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK!");
        return;
    }
    init_app();
}

fn init_app() {
    let application = gtk::Application::new(Some("toy.browser"), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    // Add CSS to our window
    application.connect_startup(|app| {
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Create our GUI
        init_window(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn init_window(application: &gtk::Application) {
    // Draw window
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Rusty Browse");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    // Draw container
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Draw chrome for browser


    // Add the container to the inside of our window
    window.add(&container);

    application.connect_activate(move |_| {
        window.show_all();
    });
}


