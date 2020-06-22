extern crate bytemuck;
extern crate image;
extern crate wgpu;

/// We re-export the 'winit' crate so that downstream can have
/// access to a version of winit that this library uses
pub extern crate winit;

macro_rules! err {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        return Err(crate::A2DError::new(format!($fmt $(, $args)*), None));
    };
}

mod color;
mod error;
mod g2d;
mod res;
mod shaders;

pub use color::Color;
pub use error::A2DError;
pub use error::Result;
pub use g2d::Graphics2D;
pub use g2d::Instance;
pub use g2d::Rect;
pub use g2d::SpriteBatch;
pub use g2d::SpriteSheet;
