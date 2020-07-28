use std::ops;

/// Dimensions of a rectangle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl<D: Into<Dimensions>> ops::Add<D> for Dimensions {
    type Output = Self;

    fn add(self, other: D) -> Self {
        let other = other.into();
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl ops::Mul<f32> for Dimensions {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            width: self.width * other,
            height: self.height * other,
        }
    }
}

impl<D: Into<Dimensions>> ops::Mul<D> for Dimensions {
    type Output = Self;

    fn mul(self, other: D) -> Self {
        let other = other.into();
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl ops::Div<f32> for Dimensions {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            width: self.width / other,
            height: self.height / other,
        }
    }
}

impl<D: Into<Dimensions>> ops::Div<D> for Dimensions {
    type Output = Self;

    fn div(self, other: D) -> Self {
        let other = other.into();
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl Dimensions {
    pub fn to_array(&self) -> [f32; 2] {
        [self.width, self.height]
    }
}

impl From<[f32; 2]> for Dimensions {
    fn from(d: [f32; 2]) -> Self {
        Self {
            width: d[0],
            height: d[1],
        }
    }
}

impl<D: Into<Dimensions>> ops::Sub<D> for Dimensions {
    type Output = Self;

    fn sub(self, other: D) -> Self {
        let other = other.into();
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}
