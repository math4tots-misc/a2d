extern crate anyhow;
extern crate bytemuck;
extern crate futures;
extern crate image;
extern crate log;
extern crate simple_logger;
extern crate wgpu;
extern crate winit;

mod g2d;
mod inst;
mod rect;
mod sprite;

pub use g2d::Graphics2D;
pub use inst::Instance;
pub use rect::Rect;
pub use sprite::SpriteBatch;
pub use sprite::SpriteSheet;
