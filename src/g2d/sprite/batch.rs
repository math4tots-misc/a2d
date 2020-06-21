use crate::Instance;
use crate::SpriteSheet;
use std::rc::Rc;

pub struct SpriteBatch {
    sheet: Rc<SpriteSheet>,
    instances: Vec<Instance>,
}

impl SpriteBatch {
    pub fn new(sheet: Rc<SpriteSheet>) -> Self {
        Self {
            sheet,
            instances: Vec::new(),
        }
    }

    pub fn sheet(&self) -> &SpriteSheet {
        &self.sheet
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

    pub fn add(&mut self, inst: Instance) -> usize {
        let id = self.instances().len();
        self.instances.push(inst);
        id
    }
}
