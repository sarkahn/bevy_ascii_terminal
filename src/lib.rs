pub mod ascii;
pub mod border;
pub mod color;
pub mod padding;
pub mod pivot;
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
#[allow(deprecated)]
pub use border::TerminalBorder;
pub use padding::{BoxStyle, Padding};
pub use pivot::Pivot;
pub use render::{TerminalCamera, TerminalFont, TerminalMeshPivot, TerminalMeshWorldScaling};
pub use strings::{
    TerminalString, TerminalStringBuilder, Token, TokenIterator, wrap_line_count, wrap_string,
    wrap_tagged_line_count, wrap_tagged_string,
};
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
    #[allow(deprecated)]
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
    #[allow(deprecated)]
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
    #[allow(deprecated)]
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
        #[allow(deprecated)]
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

    #[allow(deprecated)]
    fn contains_point(&self, xy: impl GridPoint) -> bool {
        let xy = xy.to_ivec2();
        xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.to_ivec2()).all()
    }
}

macro_rules! impl_grid_size {
    ($type:ty) => {
        #[allow(deprecated)]
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

/// A grid point that may optionally have a pivot applied to it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[deprecated = "PivotedPoint will be removed by bevy 0.20. Use `Terminal::set_pivot` instead."]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

#[allow(deprecated)]
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
            pivot.pivot_position(grid_size.to_ivec2()) + pivot.transform_axis(self.point)
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

#[allow(deprecated)]
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

#[allow(deprecated)]
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

    #[allow(deprecated)]
    /// Returns a [GridRect] with it's position adjusted by the given amount
    pub fn translated(&self, pos: impl GridPoint) -> GridRect {
        GridRect::new(self.pos + pos.to_ivec2(), self.size)
    }

    #[allow(deprecated)]
    /// Returns a [GridRect] with each side adjusted by the given delta.
    pub fn resized(&self, delta: impl GridPoint) -> GridRect {
        GridRect::from_points(self.min() - delta.to_ivec2(), self.max() + delta.to_ivec2())
    }

    /// Resizes the rect along a given pivot point.
    pub fn resize_from_pivot(&mut self, pivot: Pivot, amount: i32) {
        let p = match pivot {
            Pivot::LeftTop => self.top_left() + ivec2(-1, 1) * amount,
            Pivot::CenterTop => self.top_left() + ivec2(0, 1) * amount,
            Pivot::RightTop => self.top_right() + ivec2(1, 1) * amount,
            Pivot::LeftCenter => self.top_left() + ivec2(-1, 0) * amount,
            Pivot::RightCenter => self.top_right() + ivec2(1, 0) * amount,
            Pivot::LeftBottom => self.bottom_left() + ivec2(-1, -1) * amount,
            Pivot::CenterBottom => self.bottom_left() + ivec2(0, -1) * amount,
            Pivot::RightBottom => self.bottom_right() + ivec2(1, -1) * amount,
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

    #[allow(deprecated)]
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

    /// Iterate over the tile positions of a single column of the rect.
    pub fn iter_column(
        &self,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_points().skip(col).step_by(self.width())
    }

    /// Iterate over the tile positions of a single row of the rect.
    pub fn iter_row(
        &self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_points()
            .skip(row * self.width())
            .take(self.width())
    }

    /// Iterate over all the tile positions of the border of the rect in clockwise
    /// order, starting from the bottom left.
    pub fn iter_border(&self) -> impl DoubleEndedIterator<Item = IVec2> {
        let left = self.iter_column(0);
        let top = self
            .iter_row(self.top_index())
            .skip(1)
            .take(self.width() - 2);
        let right = self.iter_column(self.right_index()).rev();
        let bottom = self
            .iter_row(self.bottom_index())
            .rev()
            .skip(1)
            .take(self.width() - 2);
        left.chain(top).chain(right).chain(bottom)
    }

    /// Iterate over each point of the rect.
    pub fn iter_points(&self) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        GridRectIter::new(*self)
    }

    /// Retrieve the position of the tile at the given pivot point on the rect.
    pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        self.bottom_left() + pivot.pivot_position(self.size.as_ivec2())
    }

    /// Retrieve a transformed point from a local pivot position.
    pub fn pivoted_point(&self, xy: impl Into<PivotedPoint>) -> IVec2 {
        let xy: PivotedPoint = xy.into();
        let pivot = xy.pivot.unwrap_or(Pivot::LeftBottom);
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

/// An iterator over the 2d grid points of a [GridRect].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRectIter {
    origin: IVec2,
    size: UVec2,
    head: IVec2,
    tail: IVec2,
}

impl GridRectIter {
    pub fn new(rect: GridRect) -> Self {
        Self {
            origin: rect.pos,
            size: rect.size,
            head: IVec2::ZERO,
            tail: rect.size.as_ivec2() - 1,
        }
    }

    pub fn can_iterate(&self) -> bool {
        self.head.y < self.tail.y || (self.head.y == self.tail.y && self.head.x <= self.tail.x)
    }
}

impl Iterator for GridRectIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.can_iterate() {
            return None;
        }
        let size = self.size.as_ivec2();
        let head = &mut self.head;

        let ret = self.origin + *head;
        head.x += 1;
        if head.x >= size.x {
            head.x = 0;
            head.y += 1;
        }

        Some(ret)
    }

    #[allow(deprecated)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if !self.can_iterate() {
            return (0, Some(0));
        }
        let count = self
            .tail
            .as_index(self.size)
            .saturating_sub(self.head.as_index(self.size))
            + 1;
        (count, Some(count))
    }
}

impl DoubleEndedIterator for GridRectIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let size = self.size.as_ivec2();
        let tail = &mut self.tail;
        let head = self.head;

        if tail.y < head.y || (tail.y == head.y && tail.x < head.x) {
            None
        } else {
            let ret = self.origin + *tail;
            tail.x -= 1;
            if tail.x < 0 {
                tail.x = size.x - 1;
                tail.y -= 1;
            }

            Some(ret)
        }
    }
}

impl ExactSizeIterator for GridRectIter {}

impl IntoIterator for GridRect {
    type Item = IVec2;

    type IntoIter = GridRectIter;

    fn into_iter(self) -> Self::IntoIter {
        GridRectIter::new(self)
    }
}
