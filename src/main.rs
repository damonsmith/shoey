extern crate gtk;

use std;
use std::env;
use std::sync::Arc;
use std::cell::RefCell;
use gtk::prelude::*; // Import all the basic things
use gtk::{Window, WindowType, Label, Entry};

mod command_window;
use command_window::CommandWindow;

fn main() {

    let home_string = env::var("HOME").unwrap();
    let HOME_DIR = home_string.as_str();
    println!("{}", HOME_DIR);

    gtk::init().expect("Unable to start GTK3. Error");

    let gui = Arc::new(CommandWindow::new());
    gui.start();
    gtk::main();
}