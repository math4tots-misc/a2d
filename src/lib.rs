macro_rules! err {
    ($fmt:expr $(, $args:expr)* $(,)?) => {
        return Err(crate::A2DError::new(format!($fmt $(, $args)*), None));
    };
}

mod error;
mod g2d;
mod geo;
mod res;
mod shaders;

pub use error::*;
pub use g2d::*;
pub use geo::*;
pub use shaders::*;
