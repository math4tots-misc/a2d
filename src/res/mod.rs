//! Miscellaneous resources

/// IBM Courier charmap from wikipedia
/// https://en.wikipedia.org/wiki/File:IBMCourierCharmap.svg
/// The image has been cleaned a bit for use here
pub(crate) const COURIER_CHARMAP: &[u8] = include_bytes!("courier.png");
pub(crate) const CHARMAP_NROWS: usize = 3;
pub(crate) const CHARMAP_NCOLS: usize = 32;
pub(crate) const CHAR_HEIGHT_TO_WIDTH_RATIO: f32 = 1.5;
pub(crate) const CHAR_EMPTY_SPACE_INDEX: usize = CHARMAP_NROWS * CHARMAP_NCOLS - 1;

pub(crate) fn char_to_charmap_index(c: char) -> Option<usize> {
    match c {
        _ if c >= '!' && c <= '~' => Some(c as usize - '!' as usize),
        ' ' => Some(CHAR_EMPTY_SPACE_INDEX),
        _ => None,
    }
}
