use std::{cell::RefCell, path::Path, rc::Rc};
extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
//use gtk::prelude::TextBufferExt;

fn main() {

    // Initialize GTK+
    gtk::init();
    let app = gtk::Application::new(
        Some("dev.henrybarreto.name-this-color"), // Application id
        Default::default() // Using default flags
    ).expect("Could not create the gtk aplication");

    let builder: gtk::Builder =  gtk::Builder::from_file(Path::new("windows.glade"));
              let main_window: gtk::Window = builder.get_object("main_window").expect("Could not get the object main_window");
    
  println!("1234567");
   app.connect_activate(move |app| {
     let button1: gtk::Button = builder.get_object("btnok").expect("Could not get the button");
     let text1: gtk::Entry = builder.get_object("txtinput").expect("Could not get the entry");
          
  button1.connect_clicked(move |text|{
   let name = text1.get_text().to_string(); // getting name from the entry

       println!("input={}",name);
      
    });
     
    main_window.show_all();
    //end file error
   });
   
   app.run(&[]); // initializing the application
   gtk::main(); // initializing the gtk looping
}