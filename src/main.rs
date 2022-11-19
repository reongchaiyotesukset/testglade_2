// src/main.rs
use std::{cell::RefCell, path::Path, rc::Rc};

// gtk needs
use gtk::prelude::*;
use gio::prelude::*;

use ntc::Color;

mod ntc; // importing the ntc module

fn main() {
    gtk::init() // This function will initialize the gtk
    .expect("Could not init the GTK"); 
    // and if something goes wrong, it will send this message
    /*
    The documentation says about gtk::init and gtk::Application::new:
    "When using Application, it is not necessary to call gtk_init manually. 
    It is called as soon as the application gets registered as the 
    primary instance".
    It worth to check it.
    */

    // Here it defined a gtk application, the minimum to init an application
    // There are some caveats about this
    /*
       To build this interface, I have used a component GtkWindow as father of from 
       all others components, hence, it needed to create Gtk::Application inside 
       de code.

       If a GtkApplicationWindow had been to choose, it would not be necessary, 
       because it alraedy had a Gtk::Applicaiton "inside".
   */
    let application = gtk::Application::new(
        Some("dev.henrybarreto.name-this-color"), // Application id
        Default::default() // Using default flags
    ).expect("Could not create the gtk aplication");

    // The magic happens in this line
    // The ntc.glade is pushed into our code through a builder.
    // With this builder it is possible to get all components inside the XML from Glade
    let builder: gtk::Builder =  gtk::Builder::from_file(Path::new("ntc.glade"));

    // ----------------------------------------------------------|
    let colors_saved = Rc::new(RefCell::new(ntc::NTC::new()));// |
    // ----------------------------------------------------------|

    // when the signal connect_activate was sent, the application will get our
    // components for work
    application.connect_activate(move |_| {
    // All components from the ntc.glade are imported, until the one has not used to
    // for didactic propouses
    // the "method" get_object gets from the id.
        let main_window: gtk::Window = builder.get_object("main_window").expect("Could not get the object main_window");
        let save_button: gtk::Button = builder.get_object("save_button").expect("Could not get the save_button");
        let color_selection: gtk::ColorButton = builder.get_object("color_selection").expect("Could not get the color_selection");
        let color_name_entry: gtk::Entry = builder.get_object("color_name_entry").expect("Could not get the color_name_entry");
        //let _select_color_label: gtk::Label = builder.get_object("select_color_label").expect("Could not get the select_color_label");
        //let _name_color_label: gtk::Label = builder.get_object("name_color_label").expect("Could not get the name_color_label");
        let registered_color_label: gtk::Label = builder.get_object("registered_color_label").expect("Could not get the registeredd_color_label");

        let colors_saved = colors_saved.clone();

    // When the button was clicked...
    // The "main" logic happen here
        save_button.connect_clicked(move |_| {
            let color_rgba = color_selection.get_rgba(); // getting the color from the button
            let color: Color = Color { // setting manually color by color for didactic.
                red: color_rgba.red,
                green: color_rgba.green,
                blue: color_rgba.blue,
                alpha: color_rgba.alpha
            };
            let name = color_name_entry.get_text().to_string(); // getting name from the entry

            registered_color_label.set_visible(true); // Letting the label visible
            if let Ok(()) = colors_saved.borrow_mut().save_color(color, name) { // if the color is saved correctly
                registered_color_label.set_text("Registered!");
            } else { // when does it not
                registered_color_label.set_text("Already Registered!");
            }
        });

    // "event" when the close button is clicked
        main_window.connect_destroy(move |_| {
        // the gtk application is closed
            gtk::main_quit(); 
        });

        main_window.show(); // showing all components inside the main_window
    });

    application.run(&[]); // initializing the application
    gtk::main(); // initializing the gtk looping
}
