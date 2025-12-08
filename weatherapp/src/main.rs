use leptos::prelude::*;
use weatherapp::App;
use console_log::init_with_level;
use log::Level;

fn main() {

    init_with_level(Level::Info).expect("Fehler beim Initialisieren des Loggers");

    mount_to_body(|| {
        view! { 
            <App />
        }
    });
}