use crate::Instance;
use crate::SpriteSheet;
use crate::Translation;
use std::rc::Rc;

pub struct SpriteBatch {
    sheet: Rc<SpriteSheet>,
    instances: Vec<Instance>,
    translation: Translation,
}

impl SpriteBatch {
    pub fn new(sheet: Rc<SpriteSheet>) -> Self {
        Self {
            sheet,
            instances: Vec::new(),
            translation: [0.0, 0.0],
        }
    }

    pub fn sheet(&self) -> &SpriteSheet {
        &self.sheet
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

    pub fn set(&mut self, i: usize, inst: Instance) {
        self.instances[i] = inst
    }

    pub fn add(&mut self, inst: Instance) {
        self.instances.push(inst);
    }

    pub fn last(&self) -> Option<&Instance> {
        self.instances.last()
    }

    pub fn pop(&mut self) {
        self.instances.pop();
    }
}
