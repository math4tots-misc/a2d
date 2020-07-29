use super::*;

/// Public methods of Graphics2D
impl Graphics2D {
    pub async fn new<W: HasRawWindowHandle>(width: u32, height: u32, window: &W) -> Result<Self> {
        let mut graphics = Self::new0(width, height, window).await?;
        graphics.set_scale([width as f32, height as f32]);
        Ok(graphics)
    }

    pub fn render(&mut self) -> Result<()> {
        struct BatchInfo<'a> {
            batch: &'a Batch,
            instance_buffer: wgpu::Buffer,
            translation_bind_group: wgpu::BindGroup,
        }
        let batches_with_instance_buffers = {
            let mut vec = Vec::new();
            for batch in self.batches.iter().rev().flatten() {
                // wgpu will error if you try to create a buffer of size 0,
                // so explicitly check for those cases and skip
                if batch.instances().is_empty() {
                    continue;
                }
                let instance_buffer = self.device.create_buffer_with_data(
                    bytemuck::cast_slice(batch.instances()),
                    wgpu::BufferUsage::VERTEX,
                );
                let translation_buffer = self.device.create_buffer_with_data(
                    bytemuck::cast_slice(&[batch.scale(), batch.translation()]),
                    wgpu::BufferUsage::UNIFORM,
                );
                let translation_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.translation_uniform_bind_group_layout,
                        bindings: &[wgpu::Binding {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &translation_buffer,
                                range: 0..(std::mem::size_of::<Scaling>()
                                    + std::mem::size_of::<Translation>())
                                    as wgpu::BufferAddress,
                            },
                        }],
                        label: Some("per_batch_scale_uniform_bind_group"),
                    });
                vec.push(BatchInfo {
                    batch,
                    instance_buffer,
                    translation_bind_group,
                });
            }
            vec
        };
        let scale_uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.scale_uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &self.scale_uniform_buffer,
                    range: 0..std::mem::size_of::<Scaling>() as wgpu::BufferAddress,
                },
            }],
            label: Some("default_scale_uniform_bind_group"),
        });
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting next texture");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            for info in &batches_with_instance_buffers {
                let batch = info.batch;
                let instance_buffer = &info.instance_buffer;
                let translation_bind_group = &info.translation_bind_group;
                render_pass.set_bind_group(0, batch.sheet().bind_group(), &[]);
                render_pass.set_bind_group(1, &scale_uniform_bind_group, &[]);
                render_pass.set_bind_group(2, translation_bind_group, &[]);
                render_pass.set_vertex_buffer(0, instance_buffer, 0, 0);
                render_pass.draw(0..6, 0..batch.instances().len() as u32);
            }
        }

        self.queue.submit(&[encoder.finish()]);
        Ok(())
    }

    /// Call this method to notify A2D that the window has been resized
    pub fn resized(&mut self, width: u32, height: u32) {
        self.sc_desc.width = width;
        self.sc_desc.height = height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.set_scale([width as f32, height as f32]);
        self.text_grid_dim = None;
    }

    /// By default, the screen coordinates are [0, 0] for the
    /// upper-left corner and [width, height] for the lower-right corner.
    /// The coordinates of the lower-right corner may be customized
    /// with `set_scale`. The `scale` method returns the currently
    /// set [max_x, max_y] values for the lower-right corner.
    pub fn scale(&self) -> [f32; 2] {
        self.scale
    }

    /// Sets the the scale to set the coordinates of the
    /// lower-right corner (the upper-left is always [0, 0]).
    /// See the method `scale` for more info.
    pub fn set_scale(&mut self, new_scale: [f32; 2]) {
        self.scale = new_scale;
        self.scale_uniform_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.scale),
            wgpu::BufferUsage::UNIFORM,
        );
    }

    /// Returns the number of sprites the batch at the given slot has.
    /// Panics if the slot is either out of bounds or there is no
    /// batch present at the given index
    pub fn nsprites(&self, slot: usize) -> usize {
        self.batches[slot].as_ref().unwrap().len()
    }

    /// Uses the builtin pixel batch to draw a pixel of the given color at the
    /// given location
    ///
    /// You probably never actually want this.
    /// But I like making this available for the sake of posterity.
    /// I imagine that for a lot of new programmers, they might imagine that
    /// images on the screen are drawn pixel by pixel. In practice,
    /// this just isn't practical due to the large number of pixels on a screen
    /// and the relatively limited number of CPU cycles, but having a
    /// method like this available to help beginners actually *see* how slow it
    /// is to draw pixel by pixel and how much it might heat up their computer
    /// and make their computer's fans go crazy is something I'd like to support.
    ///
    pub fn set_pixel<C: Into<Color>>(&mut self, x: usize, y: usize, color: C) -> Result<()> {
        let [width, _] = self.scale();
        let width = width as usize;
        let inst_index = y * width + x;
        self.pixel_batch()?.get(inst_index).color(color);
        Ok(())
    }

    /// Initialize the builtin text batch to cover the entire drawing area.
    ///
    /// The grid will be sized so that there will be exactly 'ncols' columns
    ///
    pub fn init_text_grid(&mut self, ncols: usize) -> Result<()> {
        let [width, height] = self.scale();
        let dest_width = width / ncols as f32;
        let dest_height = res::CHAR_HEIGHT_TO_WIDTH_RATIO * dest_width;
        let step_width = dest_width * 0.50;
        let step_height = dest_height * 0.50;
        let nrows = (height / dest_height) as usize;
        let mut descs = vec![];
        for row in 0..nrows {
            let y = step_height * (row as f32);
            for col in 0..ncols {
                let x = step_width * (col as f32);
                descs.push(SpriteDesc {
                    color: [1.0, 1.0, 1.0].into(),
                    src: res::CHAR_EMPTY_SPACE_INDEX,
                    dst: [x, y, x + dest_width, y + dest_height].into(),
                    rotate: 0.0,
                });
            }
        }
        self.batches[BATCH_SLOT_TEXT] = Some(Batch::new(
            Sheet::from_bytes(self, res::COURIER_CHARMAP)?,
            res::CHARMAP_NROWS,
            res::CHARMAP_NCOLS,
            &descs,
        ));
        self.text_grid_dim = Some(TextGridDim { nrows, ncols });
        Ok(())
    }

    pub fn draw_char(&mut self, row: usize, col: usize, ch: char) -> Result<()> {
        if self.text_grid_dim.is_none() {
            self.init_text_grid(DEFAULT_TEXT_NCOLS)?;
        }
        let TextGridDim { nrows, ncols } = self.text_grid_dim.unwrap();
        if row < nrows && col < ncols {
            let instance_index = ncols * row + col;
            if let Some(src) = res::char_to_charmap_index(ch) {
                self.text_batch()?.get(instance_index).src(src);
            }
        }
        Ok(())
    }

    pub fn draw_text(&mut self, row: usize, col: usize, text: &str) -> Result<()> {
        let chars: Vec<_> = text.chars().collect();
        for c in col..col + chars.len() {
            self.draw_char(row, c, chars[c - col])?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(super) struct TextGridDim {
    pub nrows: usize,
    pub ncols: usize,
}
