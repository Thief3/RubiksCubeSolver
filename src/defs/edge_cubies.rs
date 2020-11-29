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
//! A module for Edge Cubie Defitions

use std::cmp::Ordering;

/// A numbered enum of the edge pieces.
///
/// It is numbered to make ordered operations for permutation calculations,
/// easier to compute. The order is important.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Edge {
    UR = 0,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}

/// The main edge cubie.
///
/// # Variables
/// * `orientation` - A value of 0, 1, and 2, where 0 is the default
///     orientation, 1 a clockwise twist, and 2 an anti-clockwise twist.
/// * `coordinate` - A `Edge` that represents the cubes current position.
/// * `old_coordiante` - The `coordinate` that was last held before a move.
#[derive(Debug, Eq, Copy, Clone)]
pub struct EdgeCubie {
    pub orientation: i32,
    pub coordinate: Edge,
    pub old_coordinate: Edge,
}

impl Ord for EdgeCubie {
    fn cmp(&self, other: &EdgeCubie) -> Ordering {
        (self.coordinate as i32).cmp(&(other.coordinate as i32))
    }
}

impl PartialOrd for EdgeCubie {
    fn partial_cmp(&self, other: &EdgeCubie) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EdgeCubie {
    fn eq(&self, other: &EdgeCubie) -> bool {
        (self.coordinate as i32) == (other.coordinate as i32)
    }
}

impl EdgeCubie {
    /// Creates a new `EdgeCubie` with coordinate e.
    ///
    /// # Parameters
    /// * `e` - The default `Edge` to set.
    /// # Return
    /// * `EdgeCubie`
    pub fn new(e: Edge) -> EdgeCubie {
        let a = EdgeCubie {
            orientation: 0,
            coordinate: e,
            old_coordinate: e,
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

const F_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR,
    Edge::FL,
    Edge::UL,
    Edge::UB,
    Edge::DR,
    Edge::FR,
    Edge::DL,
    Edge::DB,
    Edge::UF,
    Edge::DF,
    Edge::BL,
    Edge::BR,
];
const F_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

const B_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR,
    Edge::UF,
    Edge::UL,
    Edge::BR,
    Edge::DR,
    Edge::DF,
    Edge::DL,
    Edge::BL,
    Edge::FR,
    Edge::FL,
    Edge::UB,
    Edge::DB,
];
const B_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

const R_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::FR,
    Edge::UF,
    Edge::UL,
    Edge::UB,
    Edge::BR,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::DR,
    Edge::FL,
    Edge::BL,
    Edge::UR,
];
const R_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const L_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR,
    Edge::UF,
    Edge::BL,
    Edge::UB,
    Edge::DR,
    Edge::DF,
    Edge::FL,
    Edge::DB,
    Edge::FR,
    Edge::UL,
    Edge::DL,
    Edge::BR,
];
const L_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const U_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UB,
    Edge::UR,
    Edge::UF,
    Edge::UL,
    Edge::DR,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::FR,
    Edge::FL,
    Edge::BL,
    Edge::BR,
];
const U_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const D_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR,
    Edge::UF,
    Edge::UL,
    Edge::UB,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::DR,
    Edge::FR,
    Edge::FL,
    Edge::BL,
    Edge::BR,
];
const D_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
