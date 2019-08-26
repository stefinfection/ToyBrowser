mod network;
mod controller;

extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK!");
    }
    let application = gtk::Application::new(Some("toy.browser"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| init(app));
    application.run(&[]);
}

fn init(app: &gtk::Application) {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    // Format body properly
    let raw_body: String = get_body();

    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Rusty Surf");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(WIDTH, HEIGHT);

    // Add vertical box to window
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);

    // Add text rendering area to vertical box
    let text_area = gtk::ComboBoxText::new();
    text_area.append_text(&raw_body);
    vbox.add(&text_area);

    // Add scrolled window to vertical box
    let scroller = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    vbox.pack_start(&scroller, true, true, 0);

    window.show_all();
}

// TODO: turn this into method for impl network
fn get_body() -> String {
    //let test_url: &str = "http://www.example.org/index.html";                      // TODO: works!
    //let test_url: &str = "http://spacejam.com/archive/spacejam/movie/jam.htm";    // TODO: works!
    let test_url: &str = "http://www.zggdwx.com/xiyou/1.html";         // TODO: does NOT work

    let url_pieces = network::parse_url(test_url, false);
    let (version, status, explanation, header_map, body) = network::request(&url_pieces.0, &url_pieces.1, &url_pieces.2);
    let raw_body = network::show(&body);

    raw_body
}




