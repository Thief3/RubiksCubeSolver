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
//! A module relating specifically to the corner pieces of the rubiks cube.
//!
//! Deals with movements and how they shift the coordinates and orientation
//! of the corner cube in question.

/// A numbered enum of the corner pieces.
///
/// It is numbered to make ordered operations for permutation calculations,
/// easier to compute. The order is important.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Corner {
    URF = 0,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}

/// The main CornerCubie struct.
///
/// # Variables
/// * `orientation` - A value of 0, 1, and 2, where 0 is the default
///     orientation, 1 a clockwise twist, and 2 an anti-clockwise twist.
/// * `coordinate` - A `Corner` that represents the cubes current position.
/// * `old_coordiante` - The `coordinate` that was last held before a move.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CornerCubie {
    pub orientation: i32,
    pub coordinate: Corner,
    pub old_coordinate: Corner,
}

impl CornerCubie {
    /// Creates a new `CornerCubie` with coordinate c.
    ///
    /// # Parameters
    /// * `c` - The default `Corner` to set.
    /// # Return
    /// * `CornerCubie`
    pub fn new(c: Corner) -> CornerCubie {
        let a = CornerCubie {
            orientation: 0,
            coordinate: c,
            old_coordinate: c,
        };

        a
    }
}

/// ***************************************************************************
/// The variables used in the generic `movement` function above. These are
/// static as they'll be called a lot and there is no reason to create them
/// each time instead of referencing these values.
///
/// Obtained from (http://kociemba.org/math/CubeDefs.htm)
/// ***************************************************************************

const F_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::UFL,
    Corner::DLF,
    Corner::ULB,
    Corner::UBR,
    Corner::URF,
    Corner::DFR,
    Corner::DBL,
    Corner::DRB,
];
const F_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [1, 2, 0, 0, 2, 1, 0, 0];

const B_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::UBR,
    Corner::DRB,
    Corner::DFR,
    Corner::DLF,
    Corner::ULB,
    Corner::DBL,
];
const B_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 1, 2, 0, 0, 2, 1];

const R_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::DFR,
    Corner::UFL,
    Corner::ULB,
    Corner::URF,
    Corner::DRB,
    Corner::DLF,
    Corner::DBL,
    Corner::UBR,
];
const R_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [2, 0, 0, 1, 1, 0, 0, 2];

const L_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::ULB,
    Corner::DBL,
    Corner::UBR,
    Corner::DFR,
    Corner::UFL,
    Corner::DLF,
    Corner::DRB,
];
const L_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 1, 2, 0, 0, 2, 1, 0];

const U_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::UBR,
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::DFR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
];
const U_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

const D_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::UBR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
    Corner::DFR,
];
const D_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

