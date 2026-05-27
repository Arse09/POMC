pub struct Color([u8; 4]);

impl Color {
    const WHITE: Self = Self::new(255, 255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}
