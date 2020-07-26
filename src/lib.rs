extern crate bytemuck;
extern crate image;
extern crate wgpu;

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
mod transform;
mod txt;

pub use color::Color;
pub use error::A2DError;
pub use error::Result;
pub use g2d::Graphics2D;
pub use g2d::Instance;
pub use g2d::Rect;
pub use g2d::SpriteBatch;
pub use g2d::SpriteBatchDesc;
pub use g2d::SpriteBatchId;
pub(crate) use g2d::SpriteMap;
pub use g2d::SpriteMapDimensions;
pub(crate) use g2d::SpriteSheet;
pub use g2d::SpriteSheetDesc;
pub use g2d::SpriteSheetId;
pub use transform::Dimensions;
pub use transform::Point;
pub use txt::TextGrid;

use transform::Scaling;
use transform::Translation;
