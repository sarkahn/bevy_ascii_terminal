//! Utilities for dealing with directions on a 2d grid.

use bevy::reflect::Reflect;

use super::IVec2;

use super::point::GridPoint;

pub const UP: IVec2 = IVec2::from_array([0, 1]);
pub const DOWN: IVec2 = IVec2::from_array([0, -1]);
pub const LEFT: IVec2 = IVec2::from_array([-1, 0]);
pub const RIGHT: IVec2 = IVec2::from_array([1, 0]);
pub const UP_LEFT: IVec2 = IVec2::from_array([-1, 1]);
pub const UP_RIGHT: IVec2 = IVec2::from_array([1, 1]);
pub const DOWN_LEFT: IVec2 = IVec2::from_array([-1, -1]);
pub const DOWN_RIGHT: IVec2 = IVec2::from_array([1, -1]);

/// Array of four orthogonal grid directions.
pub const DIR_4: &[IVec2] = &[UP, DOWN, LEFT, RIGHT];

/// Array of eight adjacent grid directions.
pub const DIR_8: &[IVec2] = &[
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

/// Four orthogonal directions on a 2d grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl From<Dir4> for IVec2 {
    fn from(d: Dir4) -> Self {
        match d {
            Dir4::Up => UP,
            Dir4::Down => DOWN,
            Dir4::Left => LEFT,
            Dir4::Right => RIGHT,
        }
    }
}

impl Dir4 {
    /// Retrieve the direction from the given point, or none if it's `[0,0]`.
    pub fn from_point(p: impl GridPoint) -> Option<Dir4> {
        match p.as_ivec2().signum().to_array() {
            [0, 1] => Some(Dir4::Up),
            [0, -1] => Some(Dir4::Down),
            [-1, 0] => Some(Dir4::Left),
            [1, 0] => Some(Dir4::Right),
            _ => None,
        }
    }

    /// Retrieve a direction from it's corresponding index.
    pub const fn from_index(i: usize) -> Option<Dir4> {
        match i {
            0 => Some(Dir4::Up),
            1 => Some(Dir4::Down),
            2 => Some(Dir4::Left),
            3 => Some(Dir4::Right),
            _ => None,
        }
    }

    /// Convert a direction to it's corresponding index.
    pub const fn as_index(&self) -> usize {
        match self {
            Dir4::Up => 0,
            Dir4::Down => 1,
            Dir4::Left => 2,
            Dir4::Right => 3,
        }
    }

    pub const fn as_ivec2(&self) -> IVec2 {
        match self {
            Dir4::Up => UP,
            Dir4::Down => DOWN,
            Dir4::Left => LEFT,
            Dir4::Right => RIGHT,
        }
    }
}

/// 8 directions on a 2d grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir8 {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Dir8 {
    /// Retrieve the direction from the given point, or none if it's (0,0).
    pub fn from_point(p: impl GridPoint) -> Option<Dir8> {
        match p.as_ivec2().signum().to_array() {
            [0, 1] => Some(Dir8::Up),
            [0, -1] => Some(Dir8::Down),
            [-1, 0] => Some(Dir8::Left),
            [1, 0] => Some(Dir8::Right),
            [-1, 1] => Some(Dir8::UpLeft),
            [1, 1] => Some(Dir8::UpRight),
            [-1, -1] => Some(Dir8::DownLeft),
            [1, -1] => Some(Dir8::DownRight),
            _ => None,
        }
    }

    /// Retrieve a direction from an index.
    pub const fn from_index(i: usize) -> Option<Dir8> {
        match i {
            0 => Some(Dir8::Up),
            1 => Some(Dir8::Down),
            2 => Some(Dir8::Left),
            3 => Some(Dir8::Right),
            4 => Some(Dir8::UpLeft),
            5 => Some(Dir8::UpRight),
            6 => Some(Dir8::DownLeft),
            7 => Some(Dir8::DownRight),
            _ => None,
        }
    }

    /// Convert a direction to it's corresponding index.
    pub const fn as_index(&self) -> usize {
        match self {
            Dir8::Up => 0,
            Dir8::Down => 1,
            Dir8::Left => 2,
            Dir8::Right => 3,
            Dir8::UpLeft => 4,
            Dir8::UpRight => 5,
            Dir8::DownLeft => 6,
            Dir8::DownRight => 7,
        }
    }
}

impl From<Dir8> for IVec2 {
    fn from(d: Dir8) -> Self {
        match d {
            Dir8::Up => UP,
            Dir8::Down => DOWN,
            Dir8::Left => LEFT,
            Dir8::Right => RIGHT,
            Dir8::UpLeft => UP_LEFT,
            Dir8::UpRight => UP_RIGHT,
            Dir8::DownLeft => DOWN_LEFT,
            Dir8::DownRight => DOWN_RIGHT,
        }
    }
}
