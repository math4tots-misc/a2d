use super::*;

pub(super) struct SpriteDesc {
    /// Index into the batch/sheet indicating which rectangle of the
    /// source sheet to use as image
    pub src: usize,

    /// Rectangle in the output to draw to
    pub dst: Rect,

    pub rotate: f32,

    /// The color factor to apply to this sprite
    pub color: Color,
}
