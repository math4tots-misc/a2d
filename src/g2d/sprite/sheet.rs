use crate::Color;
use crate::Graphics2D;
use crate::Result;
use std::rc::Rc;

/// An image loaded in GPU memory ready to be used with a SpriteBatch
pub(crate) struct SpriteSheet {
    bind_group: wgpu::BindGroup,
}

impl SpriteSheet {
    /// Creates a sprite sheet from image bytes
    ///
    /// The bytes are interpreted by passing the bytes to the
    /// `load_from_memory` function from the `image` crate
    pub(crate) fn from_bytes(state: &mut Graphics2D, diffuse_bytes: &[u8]) -> Result<Rc<Self>> {
        let diffuse_image = image::load_from_memory(diffuse_bytes)?;
        let diffuse_rgba = diffuse_image.to_rgba();
        Self::from_rbga_image(state, diffuse_rgba)
    }

    pub fn from_color<C: Into<Color>>(state: &mut Graphics2D, color: C) -> Result<Rc<Self>> {
        Self::from_colors::<C, Vec<C>>(state, 1, 1, vec![color])
    }

    pub fn from_colors<C, V>(
        state: &mut Graphics2D,
        width: u32,
        height: u32,
        colors: V,
    ) -> Result<Rc<Self>>
    where
        C: Into<Color>,
        V: IntoIterator<Item = C>,
    {
        let mut pixels = Vec::new();
        for color in colors {
            let color = color.into();
            pixels.extend(&color.to_u8_array())
        }
        assert_eq!((width * height * 4) as usize, pixels.len());
        Self::from_rgba_bytes(state, width, height, pixels)
    }

    pub fn from_rgba_bytes(
        state: &mut Graphics2D,
        width: u32,
        height: u32,
        bytes: Vec<u8>,
    ) -> Result<Rc<Self>> {
        let rgba = match image::RgbaImage::from_raw(width, height, bytes) {
            Some(img) => img,
            None => err!("Failed to create image from rgba bytes for SpriteSheet"),
        };
        Self::from_rbga_image(state, rgba)
    }

    /// This method is private because we don't want to expose the `image` crate
    /// as a dependency.
    /// The version of `image` we use might not match with the version
    /// that the binary crate uses.
    fn from_rbga_image(state: &mut Graphics2D, diffuse_rgba: image::RgbaImage) -> Result<Rc<Self>> {
        let device = state.device();
        let texture_bind_group_layout = state.texture_bind_group_layout();
        let queue = state.queue();

        let dimensions = diffuse_rgba.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let buffer = device.create_buffer_with_data(&diffuse_rgba, wgpu::BufferUsage::COPY_SRC);
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3d, we represent our 2d texture
            // by setting depth to 1.
            size: wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth: 1,
            },
            // You can store multiple textures of the same size in one
            // SpriteSheet object
            array_layer_count: 1,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // SAMPLED tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("diffuse_texture"),
        });
        {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texture_buffer_copy_encoder"),
            });

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &buffer,
                    offset: 0,
                    bytes_per_row: 4 * dimensions.0,
                    rows_per_image: dimensions.1,
                },
                wgpu::TextureCopyView {
                    texture: &diffuse_texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                size,
            );

            queue.submit(&[encoder.finish()]);
        }
        let diffuse_texture_view = diffuse_texture.create_default_view();

        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        Ok(Rc::new(Self { bind_group }))
    }

    pub(crate) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
