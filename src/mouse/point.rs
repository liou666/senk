use mouse_position::mouse_position::Mouse;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }

    pub fn from_mouse_position() -> Option<Self> {
        if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
            Some(Self::new(x, y))
        } else {
            None
        }
    }
}
