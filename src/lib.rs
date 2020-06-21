extern crate anyhow;
extern crate bytemuck;
extern crate futures;
extern crate image;
extern crate wgpu;
extern crate winit;

mod g2d;
mod res;
mod shaders;

pub use g2d::Graphics2D;
pub use g2d::Instance;
pub use g2d::Rect;
pub use g2d::SpriteBatch;
pub use g2d::SpriteSheet;
