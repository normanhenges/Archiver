mod data;
mod window;

use data::Day;
use window::Window;

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use adw::{ApplicationWindow, WindowTitle, HeaderBar, prelude::*};
use gtk::{gdk, gio, glib, prelude::*};
use gtk::{Application, ScrolledWindow, Box as GtkBox, ListBox, ListBoxRow, Label, Entry, Orientation, CssProvider};
use rusqlite::{Connection, Result};

const APP_ID: &str = "de.normanhenges.archiver-gtk";
const DATABASE_FILE: &str = "data/archiver.db";
const DB_BUILD_SQL_FILE: &str = "sql/build_db.sql";

fn main() {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    let db_result = setup_database();
    match db_result {
        Ok(conn) => {
            println!("Database connection successful");
            match conn.close() {
                Ok(_) => println!("Database connection closed successfully"),
                Err(_) => println!("Failed to close database connection"),
            }
        },
        Err(e) => {
            panic!("Database connection failed: {}", e);
        }
    }

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui_from_template);
    app.run();
}

fn setup_database() -> Result<Connection> {
    // Connect to the database
    let conn = Connection::open(DATABASE_FILE)?;

    // Read the SQL statement from the file
    let mut statement = String::new();
    let mut f = File::open(DB_BUILD_SQL_FILE).expect("Unable to open file");
    f.read_to_string(&mut statement).expect("Unable to read string");

    // Execute all SQL statements to build the database
    conn.execute_batch(&statement)?;

    Ok(conn)
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    
    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui_from_template(app: &Application) {
    // Build libadwaita window
    let window = Window::new(app);
    /*let window = ApplicationWindow::builder()
        .application(app)
        /*.title("Archiver")*/
        .default_width(800)
        .default_height(600)
        /*content(&content)*/
        .build();*/

    // Display window
    window.present();
}

fn build_ui(app: &Application) {
    // Build header bar necessary for libadwaita
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::builder().title("Archiver").build())
        .build();

    // Build main window layout
    let main_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();

    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    // Build srollable day list in left column
    let day_list_scroll_window = ScrolledWindow::builder()
        .min_content_width(200)
        .min_content_height(400)
        .vexpand(true)
        .build();
    
    let day_list = ListBox::builder()
        .vexpand(true)
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("Search for Day...")
        .build();

    day_list.append(&search_entry);
    day_list_scroll_window.set_child(Some(&day_list));

    // Build scrollable content box in right column
    let content_scroll_window = ScrolledWindow::builder()
        .min_content_width(400)
        .min_content_height(400)
        .vexpand(true)
        .build();
    let content_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();
    content_scroll_window.set_child(Some(&content_box));

    let content_header: Label = Label::builder()
        .label("Entries")
        .margin_top(12)
        .margin_bottom(12)
        .build();
    content_box.append(&content_header);

    // Add placeholder days to list
    let placeholder_day_texts = vec!["2024-06-01", "2024-06-02", "2024-06-03", "2024-06-04", "2024-06-05", "2024-06-06", "2024-06-07", "2024-06-08", "2024-06-09", "2024-06-10", "2024-06-11",
                                "2024-06-12", "2024-06-13", "2024-06-14", "2024-06-15", "2024-06-16", "2024-06-17", "2024-06-18", "2024-06-19", "2024-06-20", "2024-06-21", "2024-06-22",
                                "2024-06-23", "2024-06-24", "2024-06-25", "2024-06-26", "2024-06-27", "2024-06-28", "2024-06-29", "2024-06-30", "2024-07-01", "2024-07-02", "2024-07-03"];
    let mut placeholder_days: Vec<Day> = Vec::new();
    for day in placeholder_day_texts {
        match Day::from_string(day) {
            Ok(day_result) => {
                placeholder_days.push(day_result);
            },
            Err(error) => {
                eprintln!("Skipping day {}: {}", day, error);
                continue;
            },
        };
    }

    for day in placeholder_days {
        /*let day_option = if day.is_empty() { None } else { Some(day) };*/
        let row = ListBoxRow::builder()
            .css_classes(vec![String::from("content")])
            .build();
        let label = Label::builder()
            /*.label(day_option.unwrap_or("Unknown Day"))*/
            .label(day.to_string())
            .margin_top(12)
            .margin_bottom(12)
            .build();
        row.set_child(Some(&label));
        day_list.append(&row);
    }

    // Set up event handler
    let content_header_clone = content_header.clone();
    day_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            if let Some(label) = row.child() {
                if let Some(label_widget) = label.downcast_ref::<Label>() {
                    let day = label_widget.label();
                    content_header_clone
                        .set_label(&format!("Entries for: {}", day.as_str()));
                }
            }
        }
    });

    // Add widgets to main box
    main_box.append(&day_list_scroll_window);
    main_box.append(&content_scroll_window);
    content.append(&header_bar);
    content.append(&main_box);

    // Build libadwaita window
    let window = ApplicationWindow::builder()
        .application(app)
        /*.title("Archiver")*/
        .default_width(800)
        .default_height(600)
        .content(&content)
        .build();

    // Display window
    /*window.set_child(Some(&main_box));*/
    window.present();
}

fn build_day_panel() {

}