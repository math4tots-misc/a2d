use crate::res;
use crate::shaders;
use crate::Color;
use crate::Rect;
use crate::Result;
use crate::Scaling;
use crate::Translation;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;
use std::rc::Rc;

mod batch;
mod iface;
mod imp;
mod inst;
mod sheet;
mod sprite;

use batch::*;
use iface::*;
use inst::*;
use sheet::*;
use sprite::*;

pub const SLOT_LIMIT: usize = 16;

pub const BATCH_SLOT_TEXT: usize = 0;
pub const BATCH_SLOT_PIXEL: usize = 1;

pub const DEFAULT_TEXT_NCOLS: usize = 80;

pub struct Graphics2D {
    surface: wgpu::Surface,
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

    batches: [Option<Batch>; SLOT_LIMIT],

    pixel_instance_map: HashMap<(u32, u32), usize>,

    text_grid_dim: Option<TextGridDim>,
}
