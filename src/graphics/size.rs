use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size({}, {})", self.width, self.height)
    }
}

#[macro_export]
macro_rules! size {
    ($width:expr, $height:expr) => {
        $crate::graphics::size::Size::new($width, $height)
    };
}