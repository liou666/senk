use arboard::Clipboard;
use mouse_position::mouse_position::Mouse;
use rdev::{listen, Event, EventType};
use std::{error::Error, sync::Mutex, time::Duration};
use utils::copy;

mod utils;

pub static PREVIOUS_PRESS_TIME: Mutex<u128> = Mutex::new(0);
pub static PREVIOUS_RELEASE_POSITION: Mutex<(i32, i32)> = Mutex::new((0, 0));

struct POINT {
    x: i32,
    y: i32,
}

fn main() {
    println!("Start listening to mouse events...");

    // Start event listener
    if let Err(error) = listen(callback) {
        println!("Failed to listen: {:?}", error);
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            if button == rdev::Button::Left {
                println!("Left button pressed");
                let current_press_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let position = Mouse::get_mouse_position();
                match position {
                    Mouse::Position { x, y } => {
                        println!("x: {}, y: {}", x, y);
                        if let Ok(mut previous_release_position) = PREVIOUS_RELEASE_POSITION.lock()
                        {
                            *previous_release_position = (x, y);
                        } else {
                            eprintln!("Unable to lock Mutex");
                        }

                        // if let Ok(mut previous_press_time) = PREVIOUS_PRESS_TIME.lock() {
                        //     *previous_press_time = current_press_time;
                        // } else {
                        //     eprintln!("Unable to lock Mutex");
                        // }
                    }
                    Mouse::Error => println!("Error getting mouse position"),
                }
            }
        }
        EventType::ButtonRelease(button) => {
            if button == rdev::Button::Left {
                println!("Left button released");

                let mut end_point = POINT { x: 0, y: 0 };

                let current_release_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let position = Mouse::get_mouse_position();
                match position {
                    Mouse::Position { x, y } => {
                        println!("x: {}, y: {}", x, y);
                        if let Ok(previous_release_position) = PREVIOUS_RELEASE_POSITION.lock() {
                            let (prev_release_x, prev_release_y) = *previous_release_position;

                            let mouse_distance = (((x - prev_release_x).pow(2)
                                + (y - prev_release_y).pow(2))
                                as f64)
                                .sqrt();

                            let is_double_click =
                                current_release_time - *PREVIOUS_PRESS_TIME.lock().unwrap() < 500;

                            println!(
                                "Mouse move distance: {} - is_double_click: {}",
                                mouse_distance, is_double_click
                            );

                            let is_text_select = mouse_distance > 15.0 || is_double_click;
                            if is_text_select {
                                println!("Possible text selection event");
                                std::thread::spawn(move || {
                                    let selected_text = get_text_by_clipboard().unwrap_or_default();
                                    println!("Selected text: {}", selected_text);
                                });
                            }
                            *PREVIOUS_PRESS_TIME.lock().unwrap() = current_release_time;
                        } else {
                            eprintln!("Unable to lock Mutex");
                        }
                    }
                    Mouse::Error => println!("Error getting mouse position"),
                }

                // println!("position: {:#?}", position);
                // let (x, y): (i32, i32) = get_mouse_location().unwrap();
                // let (prev_release_x, prev_release_y) = { *PREVIOUS_RELEASE_POSITION.lock() };
                // {
                //     *PREVIOUS_RELEASE_POSITION.lock() = (x, y);
                // }

                // if let Ok(previous_press_time) = PREVIOUS_PRESS_TIME.lock() {
                //     let press_duration = current_release_time - *previous_press_time;
                //     // You can add duration judgment logic here
                //     println!("Press duration: {}ms", press_duration);

                //     // Logic to determine if it is a text selection event
                //     if press_duration > 50 && press_duration < 1000 {
                //         println!("Possible text selection event");
                //     }
                // } else {
                //     eprintln!("Unable to read Mutex");
                // }
            }
        }
        EventType::MouseMove { x, y } => {
            // If you need to handle mouse move events, you can add code here
            // println!("Mouse moved to: ({}, {})", x, y);
        }
        _ => {}
    }
}

fn get_text_by_clipboard() -> Result<String, Box<dyn Error>> {
    // Read Old Clipboard
    let old_clipboard = (Clipboard::new()?.get_text(), Clipboard::new()?.get_image());

    if copy() {
        // Read New Clipboard
        let new_text = Clipboard::new()?.get_text();

        // Create Write Clipboard
        let mut write_clipboard = Clipboard::new()?;

        match old_clipboard {
            (Ok(text), _) => {
                // Old Clipboard is Text
                write_clipboard.set_text(text)?;
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
            (_, Ok(image)) => {
                // Old Clipboard is Image
                write_clipboard.set_image(image)?;
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
            _ => {
                // Old Clipboard is Empty
                write_clipboard.clear()?;
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
        }
    } else {
        Err("Copy Failed".into())
    }
}
