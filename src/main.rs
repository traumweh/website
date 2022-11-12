#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

pub mod config;
pub mod webserver;

use crate::config::read_all;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[launch]
fn rocket() -> _ {
    let jsons = Arc::new(Mutex::new(HashMap::new()));
    let chords = Arc::new(Mutex::new(HashMap::new()));

    let jsons_thread = Arc::clone(&jsons);
    let chords_thread = Arc::clone(&chords);

    thread::spawn(move || loop {
        read_all(&jsons_thread, &chords_thread);
    });

    crate::webserver::start(&jsons, &chords)
}
