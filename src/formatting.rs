

pub enum Alignment {
    /// +X Right, +Y Down.
    TopLeft,
    /// +X Left, +Y Down. 
    TopRight,
    /// +X Right, +Y Up.
    Center,
    /// +X Right, +Y Up.
    BottomLeft,
    /// +X Left, +Y Up
    BottomRight,
}

impl Alignment {
    pub fn pivot(&self) -> Vec2 {
        match self {
            Alignment::TopLeft => Vec2::new(0.0,1.0),
            Alignment::TopRight => Vec2::new(1.0,1.0),
            Alignment::Center => Vec2::new(0.5,0.5),
            Alignment::BottomLeft => Vec2::new(0.0,0.0),
            Alignment::BottomRight => Vec2::new(1.0,0.0),
        }
    }

    /// Coridnate axis for adjusting an aligned position.
    pub fn axis(&self) -> IVec2 {
        match self {
            Alignment::TopLeft => IVec2::new(1,-1),
            Alignment::TopRight => IVec2::new(-1,-1),
            Alignment::Center => IVec2::new(1,1),
            Alignment::BottomLeft => IVec2::new(1,1),
            Alignment::BottomRight => IVec2::new(-1,1),
        }
    }

    /// Transform a point to it's equivalent from the perspective of this pivot.
    pub fn pivot_aligned_point(&self, point: [i32;2], size: [u32;2]) -> IVec2 {
        let point = IVec2::from(point);
        let align_offset = UVec2::from(size).as_vec2() - Vec2::ONE;
        let align_offset = (align_offset * self.pivot()).as_ivec2();
        let point = point * self.axis() + align_offset;
        point
    }
}

pub struct CharFormatter {
    tile: Tile,
    alignment: Alignment,
}

impl CharFormatter {
    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.tile.fg_color = color.into();
        self
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.tile.fg_color = color.into();
        self
    }

    pub fn with_glyph(mut self, glyph: char) -> Self {
        self.tile.glyph = glyph;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

#[test]
fn axis() {
    let xy: [i32;2] = Alignment::TopLeft.pivot_aligned_point([0,9], [10,10]).into();
    assert_eq!([0,0], xy);
    let xy: [i32;2] = Alignment::TopRight.pivot_aligned_point([9,9], [10,10]).into();
    assert_eq!([0,0], xy);
    let xy: [i32;2] = Alignment::BottomLeft.pivot_aligned_point([0,0], [10,10]).into();
    assert_eq!([0,0], xy);
    let xy: [i32;2] = Alignment::BottomRight.pivot_aligned_point([9,0], [10,10]).into();
    assert_eq!([0,0], xy);

    let xy: [i32;2] = Alignment::TopLeft.pivot_aligned_point([-1,10], [10,10]).into();
    assert_eq!([-1,-1], xy);
    let xy: [i32;2] = Alignment::TopRight.pivot_aligned_point([10,10], [10,10]).into();
    assert_eq!([-1,-1], xy);
    let xy: [i32;2] = Alignment::BottomLeft.pivot_aligned_point([-1,-1], [10,10]).into();
    assert_eq!([-1,-1], xy);
    let xy: [i32;2] = Alignment::BottomRight.pivot_aligned_point([10,-1], [10,10]).into();
    assert_eq!([-1,-1], xy);

    let xy: [i32;2] = Alignment::Center.pivot_aligned_point([0,0], [10,10]).into();
    assert_eq!([4,4], xy);
    let xy: [i32;2] = Alignment::Center.pivot_aligned_point([-1,-1], [10,10]).into();
    assert_eq!([3,3], xy);
    let xy: [i32;2] = Alignment::Center.pivot_aligned_point([1,1], [10,10]).into();
    assert_eq!([5,5], xy);
}