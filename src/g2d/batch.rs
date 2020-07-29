use super::*;
use std::rc::Rc;

pub(super) struct Batch {
    sheet: Rc<Sheet>,
    instance_buffer: wgpu::Buffer,
    scale: Scaling,
    translation: Translation,
    nrows: usize,
    ncols: usize,
    len: usize,

    pending_updates: Vec<(usize, SpriteUpdate)>,
}

#[allow(dead_code)]
impl Batch {
    pub fn new(
        graphics: &mut Graphics2D,
        sheet: Rc<Sheet>,
        nrows: usize,
        ncols: usize,
        descs: &[SpriteDesc],
    ) -> Self {
        let mut instances = vec![];
        for desc in descs {
            let src = src_index_to_rect(nrows, ncols, desc.src);
            instances.push(
                Instance::builder()
                    .src(src)
                    .dest(desc.dst)
                    .rotate(desc.rotate)
                    .color_factor(desc.color)
                    .build(),
            );
        }

        let instance_buffer = graphics.device.create_buffer_with_data(
            bytemuck::cast_slice(&instances),
            wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::MAP_WRITE,
        );

        Self {
            sheet,
            instance_buffer,
            scale: [1.0, 1.0],
            translation: [0.0, 0.0],
            nrows,
            ncols,
            len: instances.len(),
            pending_updates: vec![],
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

    pub fn get(&mut self, i: usize) -> SpriteView {
        SpriteView { batch: self, i }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub async fn flush(&mut self) -> Result<()> {
        let updates = std::mem::replace(&mut self.pending_updates, vec![]);
        if updates.is_empty() {
            return Ok(());
        }
        let min_i = updates.iter().map(|(i, _)| *i).min().unwrap();
        let max_i = updates.iter().map(|(i, _)| *i).max().unwrap();
        let mut inst_mapping = self
            .instance_buffer
            .map_write(
                (min_i * std::mem::size_of::<Instance>()) as wgpu::BufferAddress,
                ((max_i - min_i + 1) * std::mem::size_of::<Instance>()) as wgpu::BufferAddress,
            )
            .await?;
        let inst_arr = inst_mapping.as_slice();
        for (i, update) in updates {
            let start = (i - min_i) * std::mem::size_of::<Instance>();
            let end = (i - min_i + 1) * std::mem::size_of::<Instance>();
            let inst: &mut Instance = bytemuck::from_bytes_mut(&mut inst_arr[start..end]);
            match update {
                SpriteUpdate::Src(src) => inst.set_src(src),
                SpriteUpdate::Dst(dst) => inst.set_dest(dst),
                SpriteUpdate::Rotate(rot) => inst.set_rotation(rot),
                SpriteUpdate::Color(color) => inst.set_color_factor(color),
            }
        }
        Ok(())
    }
}

fn src_index_to_rect(nrows: usize, ncols: usize, index: usize) -> Rect {
    let rwidth = 1.0 / (ncols as f32);
    let rheight = 1.0 / (nrows as f32);
    let col = (index % ncols) as f32;
    let row = (index / ncols) as f32;
    [
        col * rwidth,
        row * rheight,
        (col + 1.0) * rwidth,
        (row + 1.0) * rheight,
    ]
    .into()
}
pub(super) struct SpriteView<'a> {
    batch: &'a mut Batch,
    i: usize,
}

#[allow(dead_code)]
impl<'a> SpriteView<'a> {
    pub fn src(&mut self, src_index: usize) -> &mut Self {
        let src = src_index_to_rect(self.batch.nrows, self.batch.ncols, src_index);
        self.batch
            .pending_updates
            .push((self.i, SpriteUpdate::Src(src)));
        self
    }

    pub fn dst<R: Into<Rect>>(&mut self, dst: R) -> &mut Self {
        self.batch
            .pending_updates
            .push((self.i, SpriteUpdate::Dst(dst.into())));
        self
    }

    pub fn rotate(&mut self, rotate: f32) -> &mut Self {
        self.batch
            .pending_updates
            .push((self.i, SpriteUpdate::Rotate(rotate)));
        self
    }

    pub fn color<C: Into<Color>>(&mut self, color: C) -> &mut Self {
        self.batch
            .pending_updates
            .push((self.i, SpriteUpdate::Color(color.into())));
        self
    }
}

#[derive(Debug)]
pub(super) enum SpriteUpdate {
    Src(Rect),
    Dst(Rect),
    Rotate(f32),
    Color(Color),
}
