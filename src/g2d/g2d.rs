use crate::shaders;
use crate::A2DError;
use crate::Color;
use crate::Instance;
use crate::Result;
use crate::Scaling;
use crate::SpriteBatch;
use crate::SpriteSheet;
use crate::TextGrid;
use crate::Translation;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::rc::Rc;

const SHEET_LIMIT: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteSheetId(u16);

impl SpriteSheetId {
    pub fn new(id: u16) -> Result<Self> {
        if id as usize >= SHEET_LIMIT {
            Err(A2DError::new(
                format!("Invalid SpriteSheetId ({})", id),
                None,
            ))
        } else {
            Ok(Self(id))
        }
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for SpriteSheetId {
    type Error = A2DError;

    fn try_from(id: u16) -> Result<Self> {
        Self::new(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteBatchId(u16);

impl SpriteBatchId {
    pub fn new(id: u16) -> Result<Self> {
        if id as usize >= SHEET_LIMIT {
            Err(A2DError::new(
                format!("Invalid SpriteBatchId ({})", id),
                None,
            ))
        } else {
            Ok(Self(id))
        }
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for SpriteBatchId {
    type Error = A2DError;

    fn try_from(id: u16) -> Result<Self> {
        Self::new(id)
    }
}

pub struct Graphics2D {
    surface: wgpu::Surface,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    scale_uniform_bind_group_layout: wgpu::BindGroupLayout,
    translation_uniform_bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    scale: Scaling,
    scale_uniform_buffer: wgpu::Buffer,

    courier_sprite_sheet: Option<Rc<SpriteSheet>>,
    sheets: [Option<Rc<SpriteSheet>>; SHEET_LIMIT],
    batches: [Option<SpriteBatch>; SHEET_LIMIT],
}

impl Graphics2D {
    pub async fn new<W: raw_window_handle::HasRawWindowHandle>(
        width: u32,
        height: u32,
        window: &W,
    ) -> Result<Self> {
        let surface = wgpu::Surface::create(window);
        let adapter = match wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        {
            Some(adapter) => adapter,
            None => err!(""),
        };

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: width,
            height: height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // compile shaders
        let vs_data = wgpu::read_spirv(std::io::Cursor::new(shaders::VERT))?;
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(shaders::FRAG))?;
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        // sheet bind layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // scale uniform bind layout
        let scale_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
                label: Some("scale_uniform_bind_group_layout"),
            });

        // translation uniform bind layout
        let translation_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
                label: Some("translation_uniform_bind_group_layout"),
            });

        // build the pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &scale_uniform_bind_group_layout,
                    &translation_uniform_bind_group_layout,
                ],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Instance::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let scale = [1.0, 1.0];
        let scale_uniform_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(&scale), wgpu::BufferUsage::UNIFORM);

        Ok(Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            scale_uniform_bind_group_layout,
            translation_uniform_bind_group_layout,
            render_pipeline,
            texture_bind_group_layout,
            scale,
            scale_uniform_buffer,
            courier_sprite_sheet: None,
            sheets: Default::default(),
            batches: Default::default(),
        })
    }

    fn courier_sprite_sheet(&mut self) -> Result<Rc<SpriteSheet>> {
        if self.courier_sprite_sheet.is_none() {
            self.courier_sprite_sheet = Some(TextGrid::courier_sprite_sheet(self)?);
        }
        Ok(self.courier_sprite_sheet.as_ref().unwrap().clone())
    }

    /// Creates a new TextGrid instance with the builtin courier font
    /// given the width of a character block and [num_rows, num_cols]
    pub fn new_text_grid(&mut self, char_width: f32, dim: [u32; 2]) -> Result<TextGrid> {
        let sheet = self.courier_sprite_sheet()?;
        Ok(TextGrid::new(sheet, char_width, dim))
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

    pub fn render(&mut self) {
        struct BatchInfo<'a> {
            batch: &'a SpriteBatch,
            instance_buffer: wgpu::Buffer,
            translation_bind_group: wgpu::BindGroup,
        }
        let batches_with_instance_buffers = {
            let mut vec = Vec::new();
            for batch in self.batches.iter().flatten() {
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
    }

    pub(crate) fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub(crate) fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub(crate) fn texture_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }

    fn get_sprite_sheet(&self, id: SpriteSheetId) -> Option<&Rc<SpriteSheet>> {
        self.sheets[id.get() as usize].as_ref()
    }

    /// Creates a new sprite sheet and inserts it at the given slot id
    pub fn set_sheet<'a, I, E, D>(&mut self, id: I, desc: D) -> Result<()>
    where
        A2DError: From<E>,
        I: TryInto<SpriteSheetId, Error = E>,
        D: Into<SpriteSheetDesc<'a>>,
    {
        let id = id.try_into()?;
        let sheet = match desc.into() {
            SpriteSheetDesc::Clear => None,
            SpriteSheetDesc::Color(color) => Some(SpriteSheet::from_color(self, color)?),
            SpriteSheetDesc::Courier => Some(self.courier_sprite_sheet()?),
            SpriteSheetDesc::Bytes(bytes) => Some(SpriteSheet::from_bytes(self, bytes)?),
        };
        self.sheets[id.get() as usize] = sheet;
        Ok(())
    }

    pub fn set_batch<I, E, D, ED>(&mut self, id: I, desc: D) -> Result<()>
    where
        A2DError: From<E>,
        A2DError: From<ED>,
        I: TryInto<SpriteSheetId, Error = E>,
        D: TryInto<SpriteBatchDesc, Error = ED>,
    {
        let id = id.try_into()?;
        let batch = match desc.try_into()? {
            SpriteBatchDesc::Clear => None,
            SpriteBatchDesc::Sheet(sheet_id) => match self.get_sprite_sheet(sheet_id) {
                Some(sheet) => Some(SpriteBatch::new(sheet.clone())),
                None => {
                    return Err(A2DError::new(
                        format!(
                            "There is no sprite sheet at the given slot ({:?})",
                            sheet_id
                        ),
                        None,
                    ))
                }
            },
        };
        self.batches[id.get() as usize] = batch;
        Ok(())
    }

    pub fn get_batch_mut<I, E>(&mut self, id: I) -> Result<&mut SpriteBatch>
    where
        A2DError: From<E>,
        I: TryInto<SpriteSheetId, Error = E>,
    {
        let id = id.try_into()?;
        match &mut self.batches[id.get() as usize] {
            Some(batch) => Ok(batch),
            None => Err(A2DError::new(
                format!("No batch at the given slot ({:?})", id),
                None,
            )),
        }
    }
}

pub enum SpriteSheetDesc<'a> {
    /// Just remove whatever sprite sheet is currently at this location
    Clear,

    /// Single color sprite sheet
    Color(Color),

    /// The builtin courier sprite sheet for rendering basic ASCII text
    Courier,

    /// Creates a sprite sheet from image bytes
    ///
    /// The bytes are interpreted by passing the bytes to the
    /// `load_from_memory` function from the `image` crate
    Bytes(&'a [u8]),
}

impl<'a, T> From<T> for SpriteSheetDesc<'a>
where
    Color: From<T>,
{
    fn from(t: T) -> Self {
        Self::Color(t.into())
    }
}

pub enum SpriteBatchDesc {
    /// Clears the sprite batch at the given location
    Clear,

    /// Create a new sprite batch with the given sheet
    Sheet(SpriteSheetId),
}

impl TryFrom<u16> for SpriteBatchDesc {
    type Error = A2DError;

    fn try_from(i: u16) -> Result<Self> {
        Ok(Self::Sheet(i.try_into()?))
    }
}

impl TryFrom<Option<u16>> for SpriteBatchDesc {
    type Error = A2DError;

    fn try_from(i: Option<u16>) -> Result<Self> {
        match i {
            Some(i) => Self::try_from(i),
            None => Ok(Self::Clear),
        }
    }
}
