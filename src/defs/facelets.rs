//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************
//!
//! Module for Facelets level definitions.

use defs::edge_cubies::Edge;
use defs::corner_cubies::Corner;

/// ****************************************************************************
/// * Definitions
/// ****************************************************************************

pub enum Facelets {
    U = 0,
    R,
    F,
    D,
    L,
    B,
}

trait GetFacelets {
   fn get_facelet(&self) -> Facelets;
}

impl GetFacelets for char {
    fn get_facelet(&self) -> Facelets{
        match &self {
            'U' => Facelets::U,
            'R' => Facelets::R,
            'F' => Facelets::F,
            'D' => Facelets::D,
            'L' => Facelets::L,
            'B' => Facelets::B,
             _  => panic!("That's not a facelet character!!"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Red,
    Blue,
    Orange,
    Green,
    Yellow
}

impl Color {
    pub fn get_vec(&self) -> [f32; 4] {
        match self {
            Self::White  => [1.0, 1.0, 1.0, 1.0],
            Self::Red    => [1.0, 0.0, 0.0, 1.0],
            Self::Blue   => [0.0, 0.0, 1.0, 1.0],
            Self::Orange => [1.0, 0.64, 0.0, 1.0],
            Self::Green  => [0.0, 1.0, 0.0, 1.0],
            Self::Yellow => [1.0, 1.0, 0.0, 1.0],
            
        }
    }
    // @@TODO:: 
    #[allow(dead_code)]
    pub fn get_facelet(&self) -> Facelets{
        match self {
            Self::White  => Facelets::U,
            Self::Red    => Facelets::L,
            Self::Blue   => Facelets::F,
            Self::Orange => Facelets::R,
            Self::Green  => Facelets::B,
            Self::Yellow => Facelets::D,
        }        
    }
    pub fn get_char(&self) -> char{
        match self {
            Self::White  => 'U',
            Self::Red    => 'L',
            Self::Blue   => 'F',
            Self::Orange => 'R',
            Self::Green  => 'B',
            Self::Yellow => 'D',
        }        
    }
}

/// A list of all the edges and their index in face. Already in order.
const EDGE_INDEXES: [[usize; 2]; 12] = [
        [ U6, R2 ], [ U8, F2 ], [ U4, L2 ], [ U2, B2 ], [ D6, R8 ], [ D2, F8 ],
        [ D4, L8 ], [ D8, B8 ], [ F6, R4 ], [ F4, L6 ], [ B6, L4 ], [ B4, R6 ],
];
/// A pattern matching method that takes an `Edges` and returns the two
/// `Facelets` that belong to that edge.
///
/// # Parameters
/// * `e` - An edge to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 2]` - The two `Facelets` connected to `e`.
pub fn edge_colours(e: Edge) -> [Facelets; 2] {
    match e {
        Edge::UR => [Facelets::U, Facelets::R],
        Edge::UF => [Facelets::U, Facelets::F],
        Edge::UL => [Facelets::U, Facelets::L],
        Edge::UB => [Facelets::U, Facelets::B],
        Edge::DR => [Facelets::D, Facelets::R],
        Edge::DF => [Facelets::D, Facelets::F],
        Edge::DL => [Facelets::D, Facelets::L],
        Edge::DB => [Facelets::D, Facelets::B],
        Edge::FR => [Facelets::F, Facelets::R],
        Edge::FL => [Facelets::F, Facelets::L],
        Edge::BL => [Facelets::B, Facelets::L],
        Edge::BR => [Facelets::B, Facelets::R],
    }
}

/// A list of all the corners and their indexes in `Face`. Already in order.
const CORNER_INDEXES: [[usize; 3]; 8] = [
    [ U9, R1, F3 ], [ U7, F1, L3 ], [ U1, L1, B3 ], [ U3, B1, R3 ],
    [ D3, F9, R7 ], [ D1, L9, F7 ], [ D7, B9, L7 ], [ D9, R9, B7 ],
];

/// A pattern matching method that takes a `Corners` and returns the three
/// `Facelets` that belong to that corner.
///
/// # Parameters
/// * `c` - An corner to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 3]` - The two `Facelets` connected to `c`.
pub fn corner_colours(c: Corner) -> [Facelets; 3] {
    match c {
        Corner::URF => [Facelets::U, Facelets::R, Facelets::F],
        Corner::UFL => [Facelets::U, Facelets::F, Facelets::L],
        Corner::ULB => [Facelets::U, Facelets::L, Facelets::B],
        Corner::UBR => [Facelets::U, Facelets::B, Facelets::R],
        Corner::DFR => [Facelets::D, Facelets::F, Facelets::R],
        Corner::DLF => [Facelets::D, Facelets::L, Facelets::F],
        Corner::DBL => [Facelets::D, Facelets::B, Facelets::L],
        Corner::DRB => [Facelets::D, Facelets::R, Facelets::B],
    }
}

const CORNER_LIST: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::UBR,
    Corner::DFR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
];

const EDGE_LIST: [Edge; 12] = [
    Edge::UR,
    Edge::UF,
    Edge::UL,
    Edge::UB,
    Edge::DR,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::FR,
    Edge::FL,
    Edge::BL,
    Edge::BR,
];

pub const U1: usize = 0;
pub const U2: usize = 1;
pub const U3: usize = 2;
pub const U4: usize = 3;
pub const U5: usize = 4;
pub const U6: usize = 5;
pub const U7: usize = 6;
pub const U8: usize = 7;
pub const U9: usize = 8;
pub const R1: usize = 9;
pub const R2: usize = 10;
pub const R3: usize = 11;
pub const R4: usize = 12;
pub const R5: usize = 13;
pub const R6: usize = 14;
pub const R7: usize = 15;
pub const R8: usize = 16;
pub const R9: usize = 17;
pub const F1: usize = 18;
pub const F2: usize = 19;
pub const F3: usize = 20;
pub const F4: usize = 21;
pub const F5: usize = 22;
pub const F6: usize = 23;
pub const F7: usize = 24;
pub const F8: usize = 25;
pub const F9: usize = 26;
pub const D1: usize = 27;
pub const D2: usize = 28;
pub const D3: usize = 29;
pub const D4: usize = 30;
pub const D5: usize = 31;
pub const D6: usize = 32;
pub const D7: usize = 33;
pub const D8: usize = 34;
pub const D9: usize = 35;
pub const L1: usize = 36;
pub const L2: usize = 37;
pub const L3: usize = 38;
pub const L4: usize = 39;
pub const L5: usize = 40;
pub const L6: usize = 41;
pub const L7: usize = 42;
pub const L8: usize = 43;
pub const L9: usize = 44;
pub const B1: usize = 45;
pub const B2: usize = 46;
pub const B3: usize = 47;
pub const B4: usize = 48;
pub const B5: usize = 49;
pub const B6: usize = 50;
pub const B7: usize = 51;
pub const B8: usize = 52;
pub const B9: usize = 53;
