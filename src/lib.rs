extern crate anyhow;
extern crate bytemuck;
extern crate futures;
extern crate image;
extern crate wgpu;
extern crate winit;

macro_rules! err {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        return Err(crate::A2DError::new(format!($fmt $(, $args)*), None));
    };
}

mod error;
mod g2d;
mod res;
mod shaders;

pub use error::A2DError;
pub use error::Result;
pub use g2d::Graphics2D;
pub use g2d::Instance;
pub use g2d::Rect;
pub use g2d::SpriteBatch;
pub use g2d::SpriteSheet;
