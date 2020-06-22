pub struct Color([f32; 4]);

impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        self.0
    }

    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }

    pub fn to_u8_array(&self) -> [u8; 4] {
        fn translate(x: f32) -> u8 {
            let x = if x < 0.0 {
                0.0
            } else if x > 1.0 {
                1.0
            } else {
                x
            };
            (x * 255.0) as u8
        }
        let (r, g, b, a) = self.unpack();
        [translate(r), translate(g), translate(b), translate(a)]
    }
}

impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Self(c)
    }
}

impl From<[f32; 3]> for Color {
    fn from(c: [f32; 3]) -> Self {
        Self([c[0], c[1], c[2], 1.0])
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32, f32)) -> Self {
        Self([c.0, c.1, c.2, c.3])
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32)) -> Self {
        Self([c.0, c.1, c.2, 1.0])
    }
}
