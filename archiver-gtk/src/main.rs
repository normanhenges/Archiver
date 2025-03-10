use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use adw::prelude::*;
use adw::{ApplicationWindow, WindowTitle, HeaderBar};
use gtk::{gdk, glib, prelude::*};
use gtk::{Application, ScrolledWindow, Box as GtkBox, ListBox, ListBoxRow, Label, Entry, Orientation, CssProvider};
use rusqlite::{Connection, Result};

const APP_ID: &str = "de.normanhenges.archiver-gtk";
const DATABASE_FILE: &str = "data/archiver.db";
const DB_BUILD_SQL_FILE: &str = "sql/build_db.sql";

fn main() {
    let db_result = setup_database();
    match db_result {
        Ok(conn) => {
            println!("Database connection successful");
            conn.close();
        },
        Err(e) => {
            println!("Database connection failed: {}", e);
            return;
        }
    }

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
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

fn build_ui(app: &Application) {
    // Build header bar necessary for libadwaita
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::builder().title("Archiver").build())
        .build();

    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    let main_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(200)  // Minimum width of left column
        .min_content_height(400) // Minimum height (optional)
        .vexpand(true)           // Expand vertically
        .build();
    
    let day_list = ListBox::builder()
        .vexpand(true)
        .build();

    let search_entry = Entry::builder()
        .placeholder_text("Search for Day...")
        .build();

    // Build widgets
    day_list.append(&search_entry);
    /*day_list.append(&scrolled_window);*/
    /*scrolled_window.append(&day_list);*/
    scrolled_window.set_child(Some(&day_list)); // Add ListBox to ScrolledWindow
    main_box.append(&scrolled_window); // Add ScrolledWindow to main box

    // Pack content_label inside Rc<RefCell<...>>
    let content_label = Rc::new(RefCell::new(Label::new(Some("Please select a Day from the left column."))));

    // Add placeholder days
    let placeholder_days = vec!["2024-06-01", "2024-06-02", "2024-06-03", "2024-06-04", "2024-06-05", "2024-06-06", "2024-06-07", "2024-06-08", "2024-06-09", "2024-06-10", "2024-06-11",
                                "2024-06-12", "2024-06-13", "2024-06-14", "2024-06-15", "2024-06-16", "2024-06-17", "2024-06-18", "2024-06-19", "2024-06-20", "2024-06-21", "2024-06-22",
                                "2024-06-23", "2024-06-24", "2024-06-25", "2024-06-26", "2024-06-27", "2024-06-28", "2024-06-29", "2024-06-30", "2024-07-01", "2024-07-02", "2024-07-03"];
    for day in placeholder_days {
        let day_option = if day.is_empty() { None } else { Some(day) };
        let row = ListBoxRow::builder()
            .css_classes(vec![String::from("content")])
            .build();
        let label = Label::builder()
            .label(day_option.unwrap_or("Unknown Day"))
            .margin_top(12)
            .margin_bottom(12)
            .build();
        row.set_child(Some(&label));
        day_list.append(&row);
    }

    // Clone Rc for the Closure
    let content_label_clone = Rc::clone(&content_label);
    day_list.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            if let Some(label) = row.child() {
                if let Some(label_widget) = label.downcast_ref::<Label>() {
                    let day = label_widget.label();
                    content_label_clone
                        .borrow()
                        .set_label(&format!("Entries for: {}", day.as_str()));
                }
            }
        }
    });

    // Add widgets to main box
    main_box.append(&day_list);
    main_box.append(&*content_label.borrow());
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