use crate::Dimensions;
use crate::Instance;
use crate::Point;
use crate::Rect;
use crate::SpriteBatch;
use crate::SpriteSheet;
use crate::Translation;
use std::rc::Rc;

/// A SpriteMap wraps a SpriteBatch assuming that the underlying
/// sheet is made up of equal sized rectangles
pub struct SpriteMap {
    batch: SpriteBatch,

    /// The [num_rows, num_cols] describing how the sprite sheet is divided
    dimensions: SpriteMapDimensions,

    /// The default [width, height] dimensions of the destination rectangle
    /// to use when drawing an instance
    /// This way, when drawing sprites all of the same size, you only have
    /// to specify the dimensions once.
    default_dst_dim: Dimensions,

    /// The amount to trim border by as a factor between 0 and 1, with
    /// 1 trimming everything down to a point, and 0 trimming nothing
    /// around the border
    border_trim_factor: f32,
}

impl SpriteMap {
    pub fn new<D, DD>(
        sheet: Rc<SpriteSheet>,
        dimensions: D,
        default_dst_dim: DD,
        border_trim_factor: f32,
    ) -> Self
    where
        D: Into<SpriteMapDimensions>,
        DD: Into<Dimensions>,
    {
        assert!(0.0 <= border_trim_factor && border_trim_factor <= 1.0);
        let batch = SpriteBatch::new(sheet);
        Self {
            batch,
            dimensions: dimensions.into(),
            default_dst_dim: default_dst_dim.into(),
            border_trim_factor,
        }
    }

    /// Returns the number of rows in this map
    pub fn nrows(&self) -> u32 {
        self.dimensions.nrows()
    }

    /// Returns the number of columns in this map
    pub fn ncols(&self) -> u32 {
        self.dimensions.ncols()
    }

    /// Returns the cell index of the cell at the given row and column
    pub fn cell_index(&self, row_col: [u32; 2]) -> u32 {
        self.dimensions.cell_index(row_col)
    }

    /// Returns the (row, col) coordinates of the cell given its index
    pub fn cell_coord(&self, cell_index: u32) -> [u32; 2] {
        self.dimensions.cell_coord(cell_index)
    }

    /// Sets the sprite at instance index i to use the sprite cell indicated by the given cell_index
    pub fn set_cell(&mut self, instance_index: usize, cell_index: u32) {
        let src_rect = self.rect_for_cell(cell_index);
        self.batch
            .get_mut(instance_index)
            .set_src(src_rect);
    }

    /// Adds a new instance using the image from the cell_index to a rectangle
    /// located at the given center
    pub fn add<P: Into<Point>>(&mut self, center: P, cell_index: u32) {
        let center = center.into();
        let src_rect = self.rect_for_cell(cell_index);
        let dst_rect = self.dst_rect(center);
        self.batch.add(Instance::new(src_rect, dst_rect, 0.0));
    }

    fn rect_for_cell(&self, cell_index: u32) -> Rect {
        self.dimensions.rect_for_cell(cell_index, self.border_trim_factor)
    }

    fn dst_rect(&self, center: Point) -> Rect {
        let half_dim = self.default_dst_dim / 2.0;
        [center - half_dim, center + half_dim].into()
    }

    pub fn move_to<P: Into<Point>>(&mut self, instance_index: usize, new_center: P) {
        let new_dst_rect = self.dst_rect(new_center.into());
        self.batch.get_mut(instance_index).set_dest(new_dst_rect);
    }

    pub fn translation(&self) -> Translation {
        self.batch.translation()
    }

    pub fn batch(&self) -> &SpriteBatch {
        &self.batch
    }

    pub fn set_translation(&mut self, translation: Translation) {
        self.batch.set_translation(translation)
    }

    pub fn cell_width(&self) -> f32 {
        self.dimensions.cell_width()
    }

    pub fn cell_height(&self) -> f32 {
        self.dimensions.cell_height()
    }
}

/// Describes the dimensions of how the underlying sprite sheet is divided
/// for this sprite map
pub struct SpriteMapDimensions {
    nrows: u32,
    ncols: u32,
}

impl SpriteMapDimensions {
    /// Returns the number of rows in this map
    pub fn nrows(&self) -> u32 {
        self.nrows
    }

    /// Returns the number of columns in this map
    pub fn ncols(&self) -> u32 {
        self.ncols
    }

    /// Returns the rect
    fn rect_for_cell(&self, cell_index: u32, border_trim_factor: f32) -> Rect {
        let [row, col] = self.cell_coord(cell_index);
        let cell_width = self.cell_width();
        let cell_height = self.cell_height();
        let ul_x = cell_width * (col as f32 + border_trim_factor / 2.0);
        let ul_y = cell_height * (row as f32 + border_trim_factor / 2.0);

        let lr_x = ul_x + cell_width * (1.0 - border_trim_factor);
        let lr_y = ul_y + cell_height * (1.0 - border_trim_factor);
        [ul_x, ul_y, lr_x, lr_y].into()
    }

    /// Returns the cell index of the cell at the given row and column
    pub fn cell_index(&self, row_col: [u32; 2]) -> u32 {
        let [row, col] = row_col;
        row * self.ncols + col
    }

    /// Sets the sprite at instance index i to use the sprite cell indicated by the given cell_index
    pub fn cell_coord(&self, cell_index: u32) -> [u32; 2] {
        let row = cell_index / self.ncols;
        let col = cell_index % self.ncols;
        [row, col]
    }

    /// Returns the width of a single cell (relative to sheet coordinates)
    fn cell_width(&self) -> f32 {
        1.0 / self.ncols as f32
    }

    /// Returns the height of a single cell (relative to sheet coordinates)
    fn cell_height(&self) -> f32 {
        1.0 / self.nrows as f32
    }
}

impl From<[u32; 2]> for SpriteMapDimensions {
    fn from(dim: [u32; 2]) -> SpriteMapDimensions {
        SpriteMapDimensions {
            nrows: dim[0],
            ncols: dim[1],
        }
    }
}
