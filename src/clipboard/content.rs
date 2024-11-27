use arboard::Clipboard;
use std::error::Error;

#[derive(Debug)]
pub struct ClipboardContent<'a> {
    pub text: Option<String>,
    pub image: Option<arboard::ImageData<'a>>,
}

impl<'a> ClipboardContent<'a> {
    pub fn save(clipboard: &mut Clipboard) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            text: clipboard.get_text().ok(),
            image: clipboard.get_image().ok(),
        })
    }

    pub fn restore(&self, clipboard: &mut Clipboard) -> Result<(), Box<dyn Error>> {
        match (self.text.as_ref(), self.image.as_ref()) {
            (Some(text), _) => clipboard.set_text(text)?,
            (_, Some(image)) => clipboard.set_image(image.clone())?,
            _ => clipboard.clear()?,
        }
        Ok(())
    }
}
