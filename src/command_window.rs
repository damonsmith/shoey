use gtk::prelude::*;
use std::process::Command;
use std::env;
use std::path::Path;

struct CommandLine {
    text: String,
    process: Command,
    output: String
}

struct Model {
    currentCommand: CommandLine,
    history: [CommandLine]
}


pub struct CommandWindow {
    window: gtk::Window,
    command_line: gtk::Entry,
    command_output: gtk::TextView,
    command_history: gtk::ListBox,
}


impl CommandWindow {
    pub fn new() -> CommandWindow {

        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("command_window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: gtk::Window = builder.get_object("command_window").unwrap();
        let current_dir: gtk::Label = builder.get_object("current_dir").unwrap();
        let command_line: gtk::Entry = builder.get_object("command_line").unwrap();
        let command_output: gtk::TextView = builder.get_object("command_output").unwrap();
        let command_history: gtk::ListBox = builder.get_object("command_history").unwrap();
        let outp = command_output.clone();
        current_dir.set_text(env::current_dir().unwrap().to_str().unwrap());

        command_line.connect_activate(move |entry| {

            let home_string = env::var("HOME").unwrap();
            let home_dir = home_string.as_str();

            let entry_text = entry.get_text().unwrap(); 
            entry.set_text("");
            let mut parts = entry_text.as_str().trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or(home_dir, |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    else {
                        current_dir.set_text(env::current_dir().unwrap().to_str().unwrap());
                    }
                },
                command => {
                    let command_output = Command::new(command)
                        .args(args)
                        .output()
                        .expect("failed to execute process");
                    outp.get_buffer().unwrap().set_text(String::from_utf8(command_output.stdout).unwrap().as_str());
                }
            }
        });

        CommandWindow {
            window,
            command_line,
            command_output,
            command_history,
        }
    }

    // Set up naming for the window and show it to the user.
    pub fn start(&self) {
        self.window.set_wmclass("Terminal", "Smart Terminal");
        self.window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
        self.window.show_all();
    }
}
