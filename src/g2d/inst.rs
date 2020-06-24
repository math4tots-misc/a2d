use crate::Color;
use crate::Rect;

/// Instance data.
/// Data passed to the GPU per sprite in a sprite batch.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Instance {
    /// [x, y] representing upper-left corner of the rectangle cropped from the source.
    /// Coordinates are between 0 and 1.
    /// Upper left corner is the origin [0, 0].
    src_ul: [f32; 2],

    /// [x, y] representing the lower-right corner of the rectangle cropped from the source.
    /// Coordinates are between 0 and 1.
    /// Upper left corner is the origin [0, 0].
    src_lr: [f32; 2],

    /// [x, y] representing upper-left corner of the destination rectangle.
    /// Coordinates are between 0 and 1.
    /// Upper left corner is the origin [0, 0].
    dst_ul: [f32; 2],

    /// [x, y] representing lower-right corner of the destination rectangle.
    /// Coordinates are between 0 and 1.
    /// Upper left corner is the origin [0, 0].
    dst_lr: [f32; 2],

    /// clockwise rotation in radians.
    /// around the center of the rectangle after moving to the destination rectangle.
    rotate: f32,

    /// Multiplied by the texture color per-fragment to get the final color returned
    /// by the fragment shader
    ///
    /// Defaults to [1.0, 1.0, 1.0, 1.0], so that the color remains unchanged
    color_factor: [f32; 4],
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

const FLOAT_SIZE: wgpu::BufferAddress = std::mem::size_of::<f32>() as wgpu::BufferAddress;

impl Instance {
    pub fn builder() -> InstanceBuilder {
        InstanceBuilder {
            src: [0.0, 0.0, 1.0, 1.0].into(),
            dest: [0.0, 0.0, 1.0, 1.0].into(),
            rotate: 0.0,
            color_factor: [1.0, 1.0, 1.0, 1.0],
        }
    }
    fn new<R1: Into<Rect>, R2: Into<Rect>>(
        src: R1,
        dest: R2,
        rotate: f32,
        color_factor: [f32; 4],
    ) -> Instance {
        let src = src.into();
        let dest = dest.into();
        Instance {
            src_ul: src.upper_left(),
            src_lr: src.lower_right(),
            dst_ul: dest.upper_left(),
            dst_lr: dest.lower_right(),
            rotate,
            color_factor,
        }
    }

    pub fn src(&self) -> Rect {
        [self.src_ul, self.src_lr].into()
    }

    pub fn set_src<R: Into<Rect>>(&mut self, rect: R) {
        let rect = rect.into();
        self.src_ul = rect.upper_left();
        self.src_lr = rect.lower_right();
    }

    pub fn dest(&self) -> Rect {
        [self.dst_ul, self.dst_lr].into()
    }

    pub fn set_dest<R: Into<Rect>>(&mut self, rect: R) {
        let rect = rect.into();
        self.dst_ul = rect.upper_left();
        self.dst_lr = rect.lower_right();
    }

    pub fn rotation(&self) -> f32 {
        self.rotate
    }

    pub fn set_rotation(&mut self, rotate: f32) {
        self.rotate = rotate;
    }

    pub fn set_color_factor<C: Into<Color>>(&mut self, color_factor: C) {
        self.color_factor = color_factor.into().to_array();
    }

    pub(super) fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        assert_eq!(
            std::mem::align_of::<Instance>(),
            std::mem::align_of::<f32>(),
        );
        assert_eq!(
            std::mem::size_of::<Instance>(),
            std::mem::size_of::<f32>() * 13,
        );
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * 2,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * (2 + 2),
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * (2 + 2 + 2),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * (2 + 2 + 2 + 2),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * (2 + 2 + 2 + 2 + 1),
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

pub struct InstanceBuilder {
    src: Rect,
    dest: Rect,
    rotate: f32,
    color_factor: [f32; 4],
}

impl InstanceBuilder {
    pub fn build(self) -> Instance {
        Instance::new(self.src, self.dest, self.rotate, self.color_factor)
    }

    pub fn src<R: Into<Rect>>(mut self, src: R) -> Self {
        self.src = src.into();
        self
    }

    pub fn dest<R: Into<Rect>>(mut self, dest: R) -> Self {
        self.dest = dest.into();
        self
    }

    pub fn rotate(mut self, rotate: f32) -> Self {
        self.rotate = rotate;
        self
    }

    /// Sets the color factor
    /// NOTE: this isn't actually the color per-se;
    /// the value passed here is multiplied with the color returned
    /// by the texture to get the color to return from the fragment
    /// shader
    pub fn color_factor<C: Into<Color>>(mut self, color_factor: C) -> Self {
        self.color_factor = color_factor.into().to_array();
        self
    }
}

impl From<InstanceBuilder> for Instance {
    fn from(builder: InstanceBuilder) -> Self {
        builder.build()
    }
}
