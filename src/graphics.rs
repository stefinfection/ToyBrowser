/* Controls drawing window and rendering graphical pieces within.
 *
 * Ref: https://github.com/gtk-rs/examples/blob/master/src/bin/css.rs
 *      https://github.com/maekawatoshiki/naglfar/blob/master/src/window.rs
 */
extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;






