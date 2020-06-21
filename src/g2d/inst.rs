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

    /// counter-clockwise rotation in radians.
    /// around the center of the rectangle after moving to the dest rectangle.
    rotate: f32,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

const FLOAT_SIZE: wgpu::BufferAddress = std::mem::size_of::<f32>() as wgpu::BufferAddress;

impl Instance {
    pub fn new<R1: Into<Rect>, R2: Into<Rect>>(src: R1, dst: R2, rotate: f32) -> Instance {
        let src = src.into();
        let dst = dst.into();
        Instance {
            src_ul: src.upper_left(),
            src_lr: src.lower_right(),
            dst_ul: dst.upper_left(),
            dst_lr: dst.lower_right(),
            rotate,
        }
    }
    pub const fn const_new(src: Rect, dst: Rect, rotate: f32) -> Instance {
        Instance {
            src_ul: src.upper_left(),
            src_lr: src.lower_right(),
            dst_ul: dst.upper_left(),
            dst_lr: dst.lower_right(),
            rotate,
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

    pub(super) fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        assert_eq!(
            std::mem::align_of::<Instance>(),
            std::mem::align_of::<f32>(),
        );
        assert_eq!(
            std::mem::size_of::<Instance>(),
            std::mem::size_of::<f32>() * 9,
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
                    offset: FLOAT_SIZE * (4 + 2),
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * (4 + 2 + 2),
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float,
                },
            ],
        }
    }
}
