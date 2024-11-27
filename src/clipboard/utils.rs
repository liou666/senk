use device_query::DeviceQuery;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub fn copy() -> bool {
    use device_query::Keycode;
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };

    // Save initial clipboard content for comparison
    let clipboard_before = arboard::Clipboard::new()
        .ok()
        .and_then(|mut clipboard| clipboard.get_text().ok());

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let device_state = device_query::DeviceState::new();
    let keys = device_state.get_keys();

    // Check for interrupt keys
    if keys.contains(&Keycode::LAlt)
        || keys.contains(&Keycode::RAlt)
        || keys.contains(&Keycode::Escape)
    {
        return false;
    }

    // Release Shift key if pressed
    if keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift) {
        enigo.key(Key::Shift, Release).unwrap();
    }

    // Simulate Ctrl+C (or Command+C) key press
    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Press).unwrap();
        enigo.key(Key::C, Click).unwrap();
        enigo.key(Key::Meta, Release).unwrap();
    }
    #[cfg(not(target_os = "macos"))]
    {
        enigo.key(Key::RControl, Press).unwrap();
        enigo.key(Key::Insert, Click).unwrap();
        enigo.key(Key::RControl, Release).unwrap();
        enigo.key(Key::Insert, Release).unwrap();
    }

    // Wait for clipboard to update
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Get updated clipboard content and compare
    let clipboard_after = arboard::Clipboard::new()
        .ok()
        .and_then(|mut clipboard| clipboard.get_text().ok());

    clipboard_before != clipboard_after
}
