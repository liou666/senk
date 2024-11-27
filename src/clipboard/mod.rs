pub mod content;
pub mod utils;

use crate::clipboard::content::ClipboardContent;
use arboard::Clipboard;
use std::error::Error;

pub fn get_text_by_clipboard() -> Result<String, Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let old_content = ClipboardContent::save(&mut clipboard)?;

    if !utils::copy() {
        return Err("Copy operation failed".into());
    }

    let new_text = clipboard.get_text()?;
    old_content.restore(&mut Clipboard::new()?)?;

    Ok(new_text)
}
