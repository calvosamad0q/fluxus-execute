use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};

pub fn create_gui() {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Fluxus Executor");
    window.set_default_size(400, 200);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&layout);

    let label = Label::new(Some("Enter your script:"));
    layout.pack_start(&label, false, false, 0);

    let script_entry = Entry::new();
    layout.pack_start(&script_entry, false, false, 0);

    let execute_button = Button::new_with_label("Execute");
    layout.pack_start(&execute_button, false, false, 0);

    let output_label = Label::new(Some(""));
    layout.pack_start(&output_label, false, false, 0);

    execute_button.connect_clicked(move |_| {
        let script = script_entry.get_text().unwrap_or_default();
        if validate_script(&script) {
            let output = run_script(&script).unwrap_or_else(|e| e);
            output_label.set_text(&output);
        } else {
            output_label.set_text("Invalid script.");
        }
    });

    window.show_all();
}