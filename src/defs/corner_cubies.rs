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
