use crate::Dimensions;
use crate::Graphics2D;
use crate::Rect;
use crate::Result;
use crate::SpriteBatch;
use crate::SpriteMap;
use crate::SpriteSheet;
use crate::Translation;
use std::rc::Rc;

/// Convenience struct for drawing text on the screen.
/// Currently, functionality is pretty limited:
///     * only able to use the Courier font bundled with A2D,
///     * only ASCII text is supported
pub struct TextGrid {
    smap: SpriteMap,
    dim: [u32; 2],
    char_dim: Dimensions,
}

impl TextGrid {
    /// The layout of how the characters should be laid out in the sprite-sheet
    const CHAR_MAP_LAYOUT_DIM: [u32; 2] = [3, 32];

    /// The width to height ratio of each drawn character rectangle
    pub const CHAR_WIDTH_TO_HEIGHT_RATIO: f32 = 3.0 / 4.0;

    /// The ratio to trim off each of the source rectangles so that the borders
    /// are not included in the draw
    pub const PADDING_FACTOR: [f32; 2] = [0.30, 0.10];

    /// loads the Courier sprite sheet embedded with A2D
    pub(crate) fn courier_sprite_sheet(graphics: &mut Graphics2D) -> Result<Rc<SpriteSheet>> {
        SpriteSheet::from_bytes(graphics, crate::res::COURIER_CHARMAP)
    }

    /// Creates a new TextGrid from:
    ///     sheet: a charmap sprite sheet
    ///     char_width: that you'd like to see for a single character
    ///         The height will automatically be set such that
    ///         the width to height ratio is 3:4.
    ///     dim: [nrows, ncols] pair indicating the number of rows and columns
    ///         in this TextGrid.
    ///
    /// By default, the TextGrid will be constructed with the upper-left corner
    /// set to the origin.
    /// You can call the `set_translation` method on it to move it to a different
    /// location if you'd like
    ///
    pub(crate) fn new(sheet: Rc<SpriteSheet>, char_width: f32, dim: [u32; 2]) -> Self {
        let [padding_factor_x, padding_factor_y] = Self::PADDING_FACTOR;
        let char_height = char_width / Self::CHAR_WIDTH_TO_HEIGHT_RATIO / (1.0 - padding_factor_x)
            * (1.0 - padding_factor_y);
        let mut smap = SpriteMap::new(
            sheet,
            Self::CHAR_MAP_LAYOUT_DIM,
            [char_width, char_height],
            Self::PADDING_FACTOR,
        );
        let [nrows, ncols] = dim;
        let empty_cell_index = Self::char_to_cell_index(' ');
        for r in 0..nrows {
            let y = char_height * (r as f32 + 0.5);
            for c in 0..ncols {
                let x = char_width * (c as f32 + 0.5);
                smap.add([x, y], empty_cell_index);
            }
        }
        Self {
            smap,
            dim,
            char_dim: [char_width, char_height].into(),
        }
    }

    /// The width of a single character cell
    pub fn char_width(&self) -> f32 {
        self.char_dim.width
    }

    /// The height of a single character cell
    pub fn char_height(&self) -> f32 {
        self.char_dim.height
    }

    pub fn char_dim(&self) -> Dimensions {
        self.char_dim
    }

    /// Gives the rectangle coordinates of where the character at given row and column
    /// is drawn
    pub fn rect_for_coord(&self, row_col: [u32; 2]) -> Rect {
        let [row, col] = row_col;
        let [char_width, char_height] = self.char_dim.to_array();
        let [offset_x, offset_y] = self.smap.translation();
        let x = char_width * col as f32 + offset_x;
        let y = char_height * row as f32 + offset_y;
        [x, y, x + char_width, y + char_height].into()
    }

    /// Writes the given string to this grid starting at the given row and column
    /// This method will not wrap the string
    pub fn write_str(&mut self, coord: [u32; 2], s: &str) {
        let [mut row, mut col] = coord;
        let [nrows, ncols] = self.dimensions();
        let mut chars = s.chars();
        while let Some(ch) = chars.next() {
            if row >= nrows {
                break;
            }
            if col < ncols {
                self.write_ch([row, col], ch);
            }
            match ch {
                '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += 1;
                }
            }
        }
    }

    pub fn write_ch(&mut self, coord: [u32; 2], ch: char) {
        let [row, col] = coord;
        let instance_index = self.coordinates_to_instance_index([row, col]);
        let cell_index = Self::char_to_cell_index(ch);
        self.smap.set_cell(instance_index, cell_index);
    }

    fn char_to_cell_index(c: char) -> u32 {
        // NOTE: assumes that '!' is the first printable character
        let c = c as u32;
        if c < '!' as u32 || c >= 127 {
            // if it's not printable ASCII, point to the last
            // cell
            let [max_r, max_c] = Self::CHAR_MAP_LAYOUT_DIM;
            max_r * max_c - 1
        } else {
            c - '!' as u32
        }
    }

    /// Returns the number of rows in this TextGrid
    pub fn nrows(&self) -> u32 {
        self.dim[0]
    }

    /// Returns the number of columns in this TextGrid
    pub fn ncols(&self) -> u32 {
        self.dim[1]
    }

    fn coordinates_to_instance_index(&self, coord: [u32; 2]) -> usize {
        let [row, col] = coord;
        (row * self.ncols() + col) as usize
    }

    /// Returns the [nrows, ncols] dimensions of this TextGrid
    pub fn dimensions(&self) -> [u32; 2] {
        self.dim
    }

    /// Get the underlying SpriteBatch associated with this TextGrid
    pub fn batch(&self) -> &SpriteBatch {
        self.smap.batch()
    }

    pub fn set_translation(&mut self, translation: Translation) {
        self.smap.set_translation(translation);
    }

    pub fn translation(&self) -> Translation {
        self.smap.translation()
    }
}
