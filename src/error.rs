use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, A2DError>;

pub struct A2DError {
    message: String,

    /// TODO: See if I can store the real Error value while keeping this
    /// compatible with anyhow::Result
    source: Option<String>,
}

impl A2DError {
    pub(crate) fn new(message: String, source: Option<Box<dyn Error>>) -> A2DError {
        A2DError { message, source: source.map(|s| format!("{:?}", s)) }
    }
}

impl Display for A2DError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "A2DError {}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl Debug for A2DError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "A2DError {}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, ": {:?}", source)?;
        }
        Ok(())
    }
}

impl Error for A2DError {}

impl From<std::io::Error> for A2DError {
    fn from(e: std::io::Error) -> Self {
        A2DError::new(format!("IOError"), Some(Box::new(e)))
    }
}

impl From<image::ImageError> for A2DError {
    fn from(e: image::ImageError) -> Self {
        A2DError::new(format!("ImageError"), Some(Box::new(e)))
    }
}
