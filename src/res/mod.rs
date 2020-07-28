//! Miscellaneous resources

/// IBM Courier charmap from wikipedia
/// https://en.wikipedia.org/wiki/File:IBMCourierCharmap.svg
/// The image has been cleaned a bit for use here
pub(crate) const COURIER_CHARMAP: &[u8] = include_bytes!("courier.png");
pub(crate) const CHARMAP_NROWS: usize = 3;
pub(crate) const CHARMAP_NCOLS: usize = 32;
