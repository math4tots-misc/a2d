use super::*;
use std::rc::Rc;

pub(super) struct Batch {
    sheet: Rc<Sheet>,
    instances: Vec<Instance>,
    scale: Scaling,
    translation: Translation,
    nrows: usize,
    ncols: usize,
}

impl Batch {
    pub fn new(sheet: Rc<Sheet>, nrows: usize, ncols: usize) -> Self {
        Self {
            sheet,
            instances: Vec::new(),
            scale: [1.0, 1.0],
            translation: [0.0, 0.0],
            nrows,
            ncols,
        }
    }

    pub fn sheet(&self) -> &Sheet {
        &self.sheet
    }

    /// The scaling that's applied before performing the batch translation
    /// This allows scaling the size of all elements in a batch at once
    /// independent of all other batches
    pub fn scale(&self) -> Scaling {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Scaling) {
        self.scale = scale
    }

    pub fn translation(&self) -> Translation {
        self.translation
    }

    pub fn set_translation(&mut self, translation: Translation) {
        self.translation = translation;
    }

    pub fn instances(&self) -> &[Instance] {
        &self.instances
    }

    pub fn clear(&mut self) {
        self.instances.clear();
    }

    pub fn get(&mut self, i: usize) -> SpriteView {
        SpriteView { batch: self, i }
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn add(&mut self, desc: SpriteDesc) {
        let src = self.src_index_to_rect(desc.src);
        self.instances.push(
            Instance::builder()
                .src(src)
                .dest(desc.dst)
                .rotate(desc.rotate)
                .color_factor(desc.color)
                .build(),
        );
    }

    fn src_index_to_rect(&self, index: usize) -> Rect {
        let rwidth = 1.0 / (self.ncols as f32);
        let rheight = 1.0 / (self.nrows as f32);
        let col = (index % self.ncols) as f32;
        let row = (index / self.ncols) as f32;
        [
            col * rwidth,
            row * rheight,
            (col + 1.0) * rwidth,
            (row + 1.0) * rheight,
        ]
        .into()
    }
}

pub(super) struct SpriteView<'a> {
    batch: &'a mut Batch,
    i: usize,
}

impl<'a> SpriteView<'a> {
    pub fn src(&mut self, src_index: usize) -> &mut Self {
        let src = self.batch.src_index_to_rect(src_index);
        self.instance().set_src(src);
        self
    }

    pub fn dst<R: Into<Rect>>(&mut self, dst: R) -> &mut Self {
        self.instance().set_dest(dst);
        self
    }

    pub fn rotate(&mut self, rotate: f32) -> &mut Self {
        self.instance().set_rotation(rotate);
        self
    }

    pub fn color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        self.instance().set_color_factor(color);
        self
    }

    fn instance(&mut self) -> &mut Instance {
        &mut self.batch.instances[self.i]
    }
}
