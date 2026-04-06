pub mod ascii;
pub mod border;
pub mod color;
pub mod padding;
pub mod render;
pub(crate) mod rexpaint;
pub mod string;
pub mod strings;
pub mod terminal;
pub mod tile;
pub mod transform;

pub use ascii::Glyph;
use bevy::{
    app::{Plugin, PostUpdate},
    math::{IVec2, UVec2, Vec2, ivec2},
    prelude::IntoScheduleConfigs,
};
pub use border::TerminalBorder;
pub use render::{TerminalCamera, TerminalFont, TerminalMeshPivot, TerminalMeshWorldScaling};
pub use strings::{StringDecorator, TerminalString};
pub use terminal::Terminal;
pub use tile::Tile;
use transform::TerminalSystemsUpdateTransform;
pub use transform::{SetTerminalGridPosition, SetTerminalLayerPosition, TerminalTransform};

pub struct TerminalPlugins;

impl Plugin for TerminalPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalMeshWorldScaling::default());
        app.add_plugins((
            transform::TerminalTransformPlugin, // 'PostUpdate' systems
            render::TerminalUvMappingPlugin,
            render::TerminalMaterialPlugin,
            render::TerminalFontPlugin,   // 'PostUpdate' systems
            render::TerminalCameraPlugin, // 'First` systems
            render::TerminalMeshPlugin,   // 'PostUpdate' systems
        ));
        app.configure_sets(
            PostUpdate,
            TerminalSystemsUpdateTransform.before(render::TerminalSystemsUpdateMesh),
        );
    }
}

/// A trait for types representing an integer point on a 2d grid.
///
/// This trait is implemented for most 2d vector types such as [IVec2], [UVec2], `[i32;2]``, etc
#[deprecated = "This trait will be removed in bevy 0.20 in favor of `impl Into<IVec2>`"]
pub trait GridPoint: Clone + Copy {
    fn xy(&self) -> IVec2;

    fn x(&self) -> i32 {
        self.xy().x
    }
    fn y(&self) -> i32 {
        self.xy().y
    }

    fn width(&self) -> usize {
        self.x() as usize
    }

    fn height(&self) -> usize {
        self.y() as usize
    }

    fn to_ivec2(&self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    fn to_uvec2(&self) -> UVec2 {
        self.to_ivec2().as_uvec2()
    }
    fn to_vec2(&self) -> Vec2 {
        self.to_ivec2().as_vec2()
    }

    fn to_array(&self) -> [i32; 2] {
        self.to_ivec2().to_array()
    }

    fn to_usize_array(&self) -> [usize; 2] {
        [self.x() as usize, self.y() as usize]
    }

    /// Calculate the 1d index of this position within a sized grid.
    ///
    /// This will panic if the grid position or the resulting 1d index is out of
    /// bounds.
    #[inline]
    fn as_index(&self, size: impl GridSize) -> usize {
        let p = self.to_ivec2();
        debug_assert!(
            p.cmpge(IVec2::ZERO).all() && p.cmplt(size.to_ivec2()).all(),
            "Attempting to convert an out of bounds grid position {:?} into a 1d index from a grid size of {}",
            self.to_array(),
            size.to_ivec2()
        );
        self.y() as usize * size.width() + self.x() as usize
    }

    /// Calculate the 1d index of this position within a sized grid.
    ///
    /// Returns [None] if the position is out of bounds.
    fn get_index(&self, size: impl GridSize) -> Option<usize> {
        let [x, y] = self.to_array();
        let [w, h] = size.to_ivec2().to_array();
        (x >= 0 && x < w && y >= 0 && y < h)
            .then(|| self.y() as usize * size.width() + self.x() as usize)
    }

    /// Returns the grid point the given number of spaces above this one.
    fn up(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() + amount)
    }

    /// Returns the grid point the given number of spaces below this one.
    fn down(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() - amount)
    }

    /// Returns the grid point the given number of spaces to the right of
    /// this one.
    fn right(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() + amount, self.y())
    }

    /// Returns the grid point the given number of spaces to the left of
    /// this one.
    fn left(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() - amount, self.y())
    }

    /// Returns this grid point offset by the given amount.
    fn offset(&self, xy: impl GridPoint) -> IVec2 {
        self.to_ivec2() + xy.to_ivec2()
    }

    fn min(&self, other: impl GridPoint) -> IVec2 {
        self.to_ivec2().min(other.to_ivec2())
    }

    fn max(&self, other: impl GridPoint) -> IVec2 {
        self.to_ivec2().max(other.to_ivec2())
    }

    /// Applies a [Pivot] to this position, which can be used to calculate a
    /// final pivot adjusted point within a sized grid.
    ///
    /// ## Example:
    ///
    /// ```
    /// use sark_grids::{GridPoint, Pivot};
    /// let point = [0,0].pivot(Pivot::TopRight);
    /// assert_eq!([8,8], point.calculate([9,9]).to_array());
    /// ```
    fn pivot(self, pivot: Pivot) -> PivotedPoint {
        PivotedPoint::new(self, pivot)
    }

    /// The [taxicab distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
    /// between two points on a four-way grid.
    #[inline]
    fn taxi_dist(self, other: impl GridPoint) -> usize {
        let d = (self.to_ivec2() - other.to_ivec2()).abs();
        (d.x + d.y) as usize
    }

    /// The [king's distance](https://en.wikipedia.org/wiki/Chebyshev_distance)
    /// between two points on an eight-way grid, assuming diagonal moves
    /// cost the same as cardinal moves.
    #[inline]
    fn king_dist(self, other: impl GridPoint) -> usize {
        let d = (self.to_ivec2() - other.to_ivec2()).abs();
        d.x.max(d.y) as usize
    }

    /// Linearly interpolate between points a and b by the amount t.
    #[inline]
    fn lerp(self, other: impl GridPoint, t: f32) -> IVec2 {
        self.to_vec2().lerp(other.to_vec2(), t).as_ivec2()
    }

    /// Whether or not the given point is cardinal (not diagonal) to this one.
    #[inline]
    fn is_cardinal(&self, other: impl GridPoint) -> bool {
        self.to_ivec2().cmpeq(other.to_ivec2()).any()
    }
}

macro_rules! impl_grid_point {
    ($type:ty) => {
        impl GridPoint for $type {
            fn xy(&self) -> IVec2 {
                IVec2::new(self[0] as i32, self[1] as i32)
            }
        }
    };
}

impl_grid_point!(IVec2);
impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);
impl_grid_point!([i32; 2]);
impl_grid_point!([usize; 2]);

/// A trait for types representing the size of a rectangular 2d grid.
#[deprecated = "This trait will be removed in bevy 0.20 in favor of `impl Into<UVec2>`"]
pub trait GridSize: Clone {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// The number of tiles for a grid of this size.
    fn tile_count(&self) -> usize {
        self.width() * self.height()
    }
    fn to_ivec2(&self) -> IVec2 {
        IVec2 {
            x: self.width() as i32,
            y: self.height() as i32,
        }
    }
    fn to_uvec2(&self) -> UVec2 {
        UVec2 {
            x: self.width() as u32,
            y: self.height() as u32,
        }
    }
    fn to_vec2(&self) -> Vec2 {
        Vec2 {
            x: (self.width() as i32) as f32,
            y: (self.height() as i32) as f32,
        }
    }
    fn to_array(&self) -> [usize; 2] {
        [self.width(), self.height()]
    }

    fn contains_point(&self, xy: impl GridPoint) -> bool {
        let xy = xy.to_ivec2();
        xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.to_ivec2()).all()
    }
}

macro_rules! impl_grid_size {
    ($type:ty) => {
        impl GridSize for $type {
            fn width(&self) -> usize {
                self[0] as usize
            }

            fn height(&self) -> usize {
                self[1] as usize
            }
        }
    };
}

impl_grid_size!(UVec2);
impl_grid_size!([u32; 2]);
impl_grid_size!([i32; 2]);
impl_grid_size!([usize; 2]);

/// A pivot on a 2d sized grid. Can be used to set positions relative to a given
/// pivot. Each pivot has it's own coordinate space it uses to calculate
/// the final adjusted position.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Pivot {
    /// Coordinate space: X increases to the right, Y increases downwards.
    TopLeft,
    /// Coordinate space: X increases to the right, Y increases downwards.
    TopCenter,
    /// Coordinate space: X increases to the left, Y increases downwards.
    TopRight,
    /// Coordinate space: X increases to the right, Y increases upwards.
    LeftCenter,
    /// Coordinate space: X increases to the left, Y increases upwards.
    RightCenter,
    /// Coordinate space: X increases to the right, Y increases upwards.
    BottomLeft,
    /// Coordinate space: X increases to the right, Y increases upwards.
    BottomCenter,
    /// Coordinate space: X increases to the left, Y increases upwards.
    BottomRight,
    /// Coordinate space: X increases to the right, Y increases upwards.
    Center,
}

impl Pivot {
    /// Coordinate axis for each pivot, used when transforming a point into
    /// the pivot's coordinate space.
    #[inline]
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
            Pivot::TopCenter => IVec2::new(1, -1),
            Pivot::LeftCenter => IVec2::new(1, 1),
            Pivot::RightCenter => IVec2::new(-1, 1),
            Pivot::BottomCenter => IVec2::new(1, 1),
        }
    }

    /// The normalized value of this pivot in default coordinate space where
    /// `[0.0, 0.0]` is the bottom left and `[1.0, 1.0]` is the top right.
    #[inline]
    pub fn normalized(&self) -> Vec2 {
        match self {
            Pivot::TopLeft => Vec2::new(0.0, 1.0),
            Pivot::TopRight => Vec2::new(1.0, 1.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::BottomLeft => Vec2::new(0.0, 0.0),
            Pivot::BottomRight => Vec2::new(1.0, 0.0),
            Pivot::TopCenter => Vec2::new(0.5, 1.0),
            Pivot::LeftCenter => Vec2::new(0.0, 0.5),
            Pivot::RightCenter => Vec2::new(1.0, 0.5),
            Pivot::BottomCenter => Vec2::new(0.5, 0.0),
        }
    }

    /// Transform a point into the pivot's coordinate space.
    #[inline]
    pub fn transform_point(&self, grid_point: impl GridPoint) -> IVec2 {
        grid_point.to_ivec2() * self.axis()
    }

    /// Calculate the position of a pivot on a sized grid.
    #[inline]
    pub fn pivot_position(&self, grid_size: impl GridSize) -> IVec2 {
        ((grid_size.to_vec2() - 1.0) * self.normalized())
            .round()
            .as_ivec2()
    }
}

/// A grid point that may optionally have a pivot applied to it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

impl PivotedPoint {
    pub fn new(xy: impl GridPoint, pivot: Pivot) -> Self {
        Self {
            point: xy.to_ivec2(),
            pivot: Some(pivot),
        }
    }

    /// Calculate the final pivoted position on a sized grid.
    ///
    /// Transforms into the pivot's coordinate space if a pivot is applied,
    /// returns the original point if no pivot is applied.
    pub fn calculate(&self, grid_size: impl GridSize) -> IVec2 {
        if let Some(pivot) = self.pivot {
            pivot.pivot_position(grid_size) + pivot.transform_point(self.point)
        } else {
            self.point
        }
    }

    /// Returns a new PivotedPoint with this point's pivot or a default applied
    /// to it if this point doesn't have one.
    pub fn with_default_pivot(&self, default_pivot: Pivot) -> PivotedPoint {
        Self {
            point: self.point,
            pivot: Some(self.pivot.unwrap_or(default_pivot)),
        }
    }
}

impl<T: GridPoint> From<T> for PivotedPoint {
    fn from(value: T) -> Self {
        Self {
            point: value.to_ivec2(),
            pivot: None,
        }
    }
}

/// A rectangle of points on a 2d grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    /// The bottom-left most tile of the rect.
    pub pos: IVec2,
    pub size: UVec2,
}

impl GridRect {
    /// Create a [GridRect] from a position (bottom left tile) and a size.
    pub fn new(pos: impl GridPoint, size: impl GridSize) -> Self {
        GridRect {
            pos: pos.to_ivec2(),
            size: size.to_uvec2(),
        }
    }

    /// Create a [GridRect] with it's center at origin (`[0,0]`).
    pub fn center_origin(size: impl GridSize) -> Self {
        Self::from_center_size([0, 0], size)
    }

    /// Create a [GridRect] from two points in space. These points define
    /// the corners of the rect, meaning they will both be contained inside the
    /// rect.
    pub fn from_points(a: impl GridPoint, b: impl GridPoint) -> Self {
        let min = a.min(b);
        let max = a.max(b);

        let size = (max - min) + 1;
        GridRect {
            pos: min,
            size: size.as_uvec2(),
        }
    }

    /// Create a [GridRect] from a center position and rect size.
    pub fn from_center_size(center: impl GridPoint, size: impl GridSize) -> Self {
        let bl = center.to_ivec2() - size.to_ivec2() / 2;
        Self::new(bl, size.to_uvec2())
    }

    /// Returns a [GridRect] clipped by the bounds of the given [GridRect]
    pub fn clipped(&self, clipper: GridRect) -> GridRect {
        let [bmin, bmax] = [clipper.min(), clipper.max()];
        let [amin, amax] = [self.min(), self.max()];
        let max = amax.min(bmax);
        let min = amin.max(bmin);
        GridRect::from_points(min, max)
    }

    /// Returns a [GridRect] with it's position adjusted by the given amount
    pub fn translated(&self, pos: impl GridPoint) -> GridRect {
        GridRect::new(self.pos + pos.to_ivec2(), self.size)
    }

    /// Returns a [GridRect] with each side adjusted by the given delta.
    pub fn resized(&self, delta: impl GridPoint) -> GridRect {
        GridRect::from_points(self.min() - delta.to_ivec2(), self.max() + delta.to_ivec2())
    }

    /// Resizes the rect along a given pivot point.
    pub fn resize_from_pivot(&mut self, pivot: Pivot, amount: i32) {
        let p = match pivot {
            Pivot::TopLeft => self.top_left() + ivec2(-1, 1) * amount,
            Pivot::TopCenter => self.top_left() + ivec2(0, 1) * amount,
            Pivot::TopRight => self.top_right() + ivec2(1, 1) * amount,
            Pivot::LeftCenter => self.top_left() + ivec2(-1, 0) * amount,
            Pivot::RightCenter => self.top_right() + ivec2(1, 0) * amount,
            Pivot::BottomLeft => self.bottom_left() + ivec2(-1, -1) * amount,
            Pivot::BottomCenter => self.bottom_left() + ivec2(0, -1) * amount,
            Pivot::BottomRight => self.bottom_right() + ivec2(1, -1) * amount,
            Pivot::Center => self.center(),
        };
        self.envelope_point(p);
    }

    /// Returns a [GridRect] with both rects contained in it.
    pub fn merged(&self, mut other: GridRect) -> GridRect {
        other.envelope_point(self.min());
        other.envelope_point(self.max());
        other
    }

    /// Adjusts a single corner of the rect to contain the given point.
    pub fn envelope_point(&mut self, point: impl GridPoint) {
        let point = point.to_ivec2();
        let min = self.min().min(point);
        let max = self.max().max(point);
        *self = GridRect::from_points(min, max);
    }

    /// Adjust this rect so the given rect is entirely contained within it.
    pub fn merge(&mut self, rect: GridRect) {
        let [min, max] = [rect.min(), rect.max()];
        self.envelope_point(min);
        self.envelope_point(max);
    }

    /// The center position of the rect.
    pub fn center(&self) -> IVec2 {
        self.pos + self.size.as_ivec2() / 2
    }

    /// The y position of the top row of the rect.
    pub fn top(&self) -> i32 {
        self.max().y
    }

    /// The y position of the bottom row of the rect.
    pub fn bottom(&self) -> i32 {
        self.min().y
    }

    /// The x position of the left column of the rect.
    pub fn left(&self) -> i32 {
        self.min().x
    }

    /// The x position of the right column of the rect.
    pub fn right(&self) -> i32 {
        self.max().x
    }

    /// The 2d position of the top left tile of the rect.
    pub fn top_left(&self) -> IVec2 {
        [self.left(), self.top()].into()
    }

    /// The 2d position of the top right tile of the rect.
    pub fn top_right(&self) -> IVec2 {
        self.max()
    }

    /// The 2d position of the bottom left tile of the rect.
    pub fn bottom_left(&self) -> IVec2 {
        self.pos
    }

    /// The 2d position of the bottom right tile of the rect.
    pub fn bottom_right(&self) -> IVec2 {
        [self.right(), self.bottom()].into()
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    /// Bottom left position of the rect.
    pub fn min(&self) -> IVec2 {
        self.pos
    }

    /// Top right position of the rect.
    pub fn max(&self) -> IVec2 {
        self.pos + (self.size.as_ivec2() - 1)
    }

    /// Index of the bottom row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn bottom_index(&self) -> usize {
        0
    }

    /// Index of the top row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn top_index(&self) -> usize {
        self.height() - 1
    }

    /// Index of the left column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn left_index(&self) -> usize {
        0
    }

    /// The index of the right most column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn right_index(&self) -> usize {
        self.width() - 1
    }

    // /// Iterate over the tile positions of a single column of the rect.
    // pub fn iter_column(
    //     &self,
    //     col: usize,
    // ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
    //     self.iter_points().skip(col).step_by(self.width())
    // }

    // /// Iterate over the tile positions of a single row of the rect.
    // pub fn iter_row(
    //     &self,
    //     row: usize,
    // ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
    //     self.iter_points()
    //         .skip(row * self.width())
    //         .take(self.width())
    // }

    // /// Iterate over all the tile positions of the border of the rect in clockwise
    // /// order, starting from the bottom left.
    // pub fn iter_border(&self) -> impl DoubleEndedIterator<Item = IVec2> {
    //     let left = self.iter_column(0);
    //     let top = self
    //         .iter_row(self.top_index())
    //         .skip(1)
    //         .take(self.width().sub(2));
    //     let right = self.iter_column(self.right_index()).rev();
    //     let bottom = self
    //         .iter_row(self.bottom_index())
    //         .rev()
    //         .skip(1)
    //         .take(self.width().sub(2));
    //     left.chain(top).chain(right).chain(bottom)
    // }

    /// Retrieve the position of the tile at the given pivot point on the rect.
    pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        self.bottom_left() + pivot.pivot_position(self.size)
    }

    /// Retrieve a transformed point from a local pivot position.
    pub fn pivoted_point(&self, xy: impl Into<PivotedPoint>) -> IVec2 {
        let xy: PivotedPoint = xy.into();
        let pivot = xy.pivot.unwrap_or(Pivot::BottomLeft);
        let xy = xy.point;
        let origin = self.pivot_point(pivot);
        origin + (xy * pivot.axis())
    }

    /// Check if a given point lies inside the rect
    #[inline]
    pub fn contains_point(&self, p: impl GridPoint) -> bool {
        let p = p.to_ivec2();
        !(p.cmplt(self.min()).any() || p.cmpgt(self.max()).any())
    }

    /// Returns true if the given rect is entirely contained within this one.
    #[inline]
    pub fn contains_rect(&self, other: GridRect) -> bool {
        other.min().cmpge(self.min()).all() && other.max().cmple(self.max()).all()
    }

    /// Check if any part of a rect overlaps another.
    #[inline]
    pub fn overlaps_rect(&self, other: GridRect) -> bool {
        self.left() <= other.right()
            && other.left() <= self.right()
            && self.bottom() <= other.top()
            && other.bottom() <= self.top()
    }
}
