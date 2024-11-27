use crate::clipboard::get_text_by_clipboard;
use crate::config::*;
use crate::mouse::point::Point;
use rdev::{listen, Event, EventType};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

static PREVIOUS_PRESS_TIME: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_PRESS_POSITION: Mutex<Point> = Mutex::new(Point::new(0, 0));
static PREVIOUS_RELEASE_POSITION: Mutex<Point> = Mutex::new(Point::new(0, 0));

pub fn start_listening() -> Result<(), String> {
    listen(handle_event).map_err(|e| format!("Failed to listen: {:?}", e))
}

fn handle_event(event: Event) {
    match event.event_type {
        EventType::ButtonPress(rdev::Button::Left) => handle_left_button_press(),
        EventType::ButtonRelease(rdev::Button::Left) => handle_left_button_release(),
        _ => {}
    }
}

fn handle_left_button_press() {
    if let Ok(mut press_pos) = PREVIOUS_PRESS_POSITION.lock() {
        if let Some(point) = Point::from_mouse_position() {
            *press_pos = point;
        }
    }
}

fn handle_left_button_release() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or_default();

    if let Some(current_point) = Point::from_mouse_position() {
        process_mouse_release(current_point, current_time);
    }
}

fn process_mouse_release(current_point: Point, current_time: u64) {
    if let (Ok(press_pos), Ok(mut release_pos)) = (
        PREVIOUS_PRESS_POSITION.lock(),
        PREVIOUS_RELEASE_POSITION.lock(),
    ) {
        let previous_time = PREVIOUS_PRESS_TIME.load(Ordering::Relaxed);
        let time_diff = current_time.saturating_sub(previous_time);

        let move_distance = press_pos.distance_to(&current_point);
        let double_click_distance = release_pos.distance_to(&current_point);

        let is_double_click = time_diff < DOUBLE_CLICK_THRESHOLD_MS
            && double_click_distance < DOUBLE_CLICK_DISTANCE_THRESHOLD;

        if move_distance > TEXT_SELECTION_DISTANCE_THRESHOLD || is_double_click {
            std::thread::spawn(|| match get_text_by_clipboard() {
                Ok(text) => println!("Selected text: {}", text),
                Err(e) => eprintln!("Failed to get text: {}", e),
            });
        }

        PREVIOUS_PRESS_TIME.store(current_time, Ordering::Relaxed);
        *release_pos = current_point;
    }
}
