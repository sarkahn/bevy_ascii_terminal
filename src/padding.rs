pub struct Padding {
    pub left: i32,
    pub bottom: i32,
    pub top: i32,
    pub right: i32,
}

impl Padding {
    pub const fn one() -> Padding {
        Self::new(1)
    }

    pub const fn new(edge_size: i32) -> Padding {
        Self {
            left: edge_size,
            right: edge_size,
            top: edge_size,
            bottom: edge_size,
        }
    }
}
