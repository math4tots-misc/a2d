use super::*;

/// Public methods of Graphics2D
impl Graphics2D {
    pub async fn new<W: HasRawWindowHandle>(width: u32, height: u32, window: &W) -> Result<Self> {
        let mut graphics = Self::new0(width, height, window).await?;

        // initialize builtin batches
        let text_batch = Batch::new(
            Sheet::from_bytes(&mut graphics, res::COURIER_CHARMAP)?,
            res::CHARMAP_NROWS,
            res::CHARMAP_NCOLS,
        );
        let pixel_batch = Batch::new(Sheet::from_color(&mut graphics, [1.0, 1.0, 1.0])?, 1, 1);
        graphics.batches[BATCH_SLOT_TEXT] = Some(text_batch);
        graphics.batches[BATCH_SLOT_PIXEL] = Some(pixel_batch);

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
    }

    /// By default, the screen coordinates are [0, 0] for the
    /// upper-left corner and [1, 1] for the lower-right corner.
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
    pub fn set_pixel<C: Into<Color>>(&mut self, xy: (u32, u32), color: C) -> Result<()> {
        let inst_index = if let Some(index) = self.pixel_instance_map.get(&xy) {
            *index
        } else {
            let index = self.pixel_instance_map.len();
            self.pixel_instance_map.insert(xy, index);
            let batch = self.pixel_batch();
            assert_eq!(batch.len(), index);
            let (x, y) = xy;
            let x = x as f32;
            let y = y as f32;
            batch.add(SpriteDesc {
                color: [0.0, 0.0, 0.0, 0.0].into(),
                src: 0,
                dst: [x, y, x + 1.0, y + 1.0].into(),
                rotate: 0.0,
            });
            index
        };
        self.pixel_batch().get(inst_index).color(color);
        Ok(())
    }
}
