use crate::Instance;
use crate::SpriteSheet;
use crate::Translation;
use crate::Scaling;
use std::rc::Rc;

pub struct SpriteBatch {
    sheet: Rc<SpriteSheet>,
    instances: Vec<Instance>,
    scale: Scaling,
    translation: Translation,
}

impl SpriteBatch {
    pub fn new(sheet: Rc<SpriteSheet>) -> Self {
        Self {
            sheet,
            instances: Vec::new(),
            scale: [1.0, 1.0],
            translation: [0.0, 0.0],
        }
    }

    pub fn sheet(&self) -> &SpriteSheet {
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

    pub fn get(&self, i: usize) -> &Instance {
        &self.instances[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut Instance {
        &mut self.instances[i]
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn set<I: Into<Instance>>(&mut self, i: usize, inst: I) {
        self.instances[i] = inst.into()
    }

    pub fn add<I: Into<Instance>>(&mut self, inst: I) {
        self.instances.push(inst.into());
    }

    pub fn last(&self) -> Option<&Instance> {
        self.instances.last()
    }

    pub fn pop(&mut self) {
        self.instances.pop();
    }
}
