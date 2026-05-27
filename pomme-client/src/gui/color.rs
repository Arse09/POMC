#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Color([u8; 4]);

impl Color {
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub fn white(alpha: f32) -> Self {
        Self::new(255, 255, 255, (alpha * 255.0) as u8)
    }

    pub fn white_i(alpha: u8) -> Self {
        Self::new(255, 255, 255, alpha)
    }

    pub fn black(alpha: f32) -> Self {
        Self::new(0, 0, 0, (alpha * 255.0) as u8)
    }

    pub fn black_i(alpha: u8) -> Self {
        Self::new(0, 0, 0, alpha)
    }

    pub fn gray(brightness: f32) -> Self {
        let c = (brightness * 255.0) as u8;
        Self::new(c, c, c, 255)
    }

    pub fn opaque(self) -> Self {
        Self::new(self.r(), self.g(), self.b(), 255)
    }

    pub fn transparent(self) -> Self {
        Self::new(self.r(), self.g(), self.b(), 0)
    }

    pub fn multiply(self, rhs: Self) -> Self {
        if self == Self::WHITE {
            return rhs;
        }
        if rhs == Self::WHITE {
            return self;
        }
        Self::new(
            (self.r() as u16 * rhs.r() as u16 / 255) as u8,
            (self.g() as u16 * rhs.g() as u16 / 255) as u8,
            (self.b() as u16 * rhs.b() as u16 / 255) as u8,
            (self.a() as u16 * rhs.a() as u16 / 255) as u8,
        )
    }

    pub fn add_rgb(self, rhs: Self) -> Self {
        Self::new(
            self.r().saturating_add(rhs.r()),
            self.g().saturating_add(rhs.g()),
            self.b().saturating_add(rhs.b()),
            self.a(),
        )
    }

    pub fn sub_rgb(self, rhs: Self) -> Self {
        Self::new(
            self.r().saturating_sub(rhs.r()),
            self.g().saturating_sub(rhs.g()),
            self.b().saturating_sub(rhs.b()),
            self.a(),
        )
    }

    pub fn multiply_alpha(self, multiplier: f32) -> Self {
        if multiplier <= 0.0 {
            return Self::new(0, 0, 0, 0);
        }
        if multiplier >= 1.0 {
            return self;
        }
        Self::new(
            self.r(),
            self.g(),
            self.b(),
            (self.a() as f32 * multiplier) as u8,
        )
    }

    pub fn scale_rgb(self, scale: f32) -> Self {
        self.scale_rgb_channels(scale, scale, scale)
    }

    pub fn scale_rgb_channels(self, sr: f32, sg: f32, sb: f32) -> Self {
        Self::new(
            (self.r() as f32 * sr).clamp(0.0, 255.0) as u8,
            (self.g() as f32 * sg).clamp(0.0, 255.0) as u8,
            (self.b() as f32 * sb).clamp(0.0, 255.0) as u8,
            self.a(),
        )
    }

    pub fn greyscale(self) -> Self {
        let g = (self.r() as f32 * 0.3 + self.g() as f32 * 0.59 + self.b() as f32 * 0.11) as u8;
        Self::new(g, g, g, self.a())
    }

    pub fn alpha_blend(dst: Self, src: Self) -> Self {
        let sa = src.a() as u16;
        let da = dst.a() as u16;
        if sa == 255 {
            return src;
        }
        if sa == 0 {
            return dst;
        }
        let a = (sa + da * (255 - sa) / 255) as u8;
        Self::new(
            Self::blend_channel(a as u16, sa, dst.r() as u16, src.r() as u16),
            Self::blend_channel(a as u16, sa, dst.g() as u16, src.g() as u16),
            Self::blend_channel(a as u16, sa, dst.b() as u16, src.b() as u16),
            a,
        )
    }

    fn blend_channel(result_a: u16, src_a: u16, dst: u16, src: u16) -> u8 {
        ((src * src_a + dst * (result_a - src_a)) / result_a) as u8
    }

    pub fn srgb_lerp(self, other: Self, t: f32) -> Self {
        Self::new(
            lerp_u8(self.r(), other.r(), t),
            lerp_u8(self.g(), other.g(), t),
            lerp_u8(self.b(), other.b(), t),
            lerp_u8(self.a(), other.a(), t),
        )
    }

    pub fn average(self, other: Self) -> Self {
        Self::new(
            ((self.r() as u16 + other.r() as u16) / 2) as u8,
            ((self.g() as u16 + other.g() as u16) / 2) as u8,
            ((self.b() as u16 + other.b() as u16) / 2) as u8,
            ((self.a() as u16 + other.a() as u16) / 2) as u8,
        )
    }

    pub fn r(self) -> u8 {
        self.0[0]
    }
    pub fn g(self) -> u8 {
        self.0[1]
    }
    pub fn b(self) -> u8 {
        self.0[2]
    }
    pub fn a(self) -> u8 {
        self.0[3]
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}
