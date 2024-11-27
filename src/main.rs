mod clipboard;
mod config;
mod mouse;

use crate::mouse::event_handler::start_listening;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start listening to mouse events...");
    start_listening()?;
    Ok(())
}
