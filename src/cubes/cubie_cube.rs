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
//! Module for Cubie level cube.

use defs::edge_cubies::Edge;
use defs::corner_cubies::Corner;


use crate::defs;
use crate::utility;

//! Struct to represent a Rubiks cube at a corner and edge level.
pub struct CubieCube {
    pub corner_permutation: [Corner; 8],
    pub corner_orientation: [usize; 8],
    
    pub edge_permutation: [Edge; 12],
    pub edge_orientation: [usize; 12]
}

impl CubieCube {

    //! Creates a new default, solved, Cube.
    pub fn new() -> CubieCube{
        CubieCube{
            corner_permutation: defs::facelets::CORNER_LIST,
            corner_orientation: [0; 8],
            edge_permutation: defs::facelets::EDGE_LIST,
            edge_orientation: [0; 12]
        }
    }

    //! Helper function for creating a new CubieCube with prechosen values.
    pub fn new_with_vals(
        cp: [Corner; 8],
        co: [usize; 8],
        ep: [Edge; 12],
        eo: [usize; 12]) -> CubieCube{
        CubieCube{
            corner_permutation: cp,
            corner_orientation: co,
            edge_permutation: ep,
            edge_orientation: eo
        }
    }
}


/// Definitions for moves

/// Upper Moves
const _CP__U: [Corner; 8] = [
    Corner::UBR,
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::DFR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
];
const _CO_U: [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
const _EP_U: [Edge; 12] = [
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
const _EP___U: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// Right Moves
const _CP_R: [Corner; 8] = [
    Corner::DFR,
    Corner::UFL,
    Corner::ULB,
    Corner::URF,
    Corner::DRB,
    Corner::DLF,
    Corner::DBL,
    Corner::UBR,
];
const _CO_R: [usize; 8] = [2, 0, 0, 1, 1, 0, 0, 2];
const _EP_R: [Edge; 12] = [
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
const _EP__R: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// Front Moves
const _CP_F: [Corner; 8] = [
    Corner::UFL,
    Corner::DLF,
    Corner::ULB,
    Corner::UBR,
    Corner::URF,
    Corner::DFR,
    Corner::DBL,
    Corner::DRB,
];
const _CO_F: [usize; 8] = [1, 2, 0, 0, 2, 1, 0, 0];
const _EP_F: [Edge; 12] = [
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
const _EP__F: [usize; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

/// Down Move
const _CP_D: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::UBR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
    Corner::DFR,
];
const _CO_D: [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
const _EP_D: [Edge; 12] = [
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
const _EP__D: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// Left Move
const _CP_L: [Corner; 8] = [
    Corner::URF,
    Corner::ULB,
    Corner::DBL,
    Corner::UBR,
    Corner::DFR,
    Corner::UFL,
    Corner::DLF,
    Corner::DRB,
];
const _CO_L: [usize; 8] = [0, 1, 2, 0, 0, 2, 1, 0];
const _EP_L: [Edge; 12] = [
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
const _EP__L: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/// Back Moves
const _CP_B: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::UBR,
    Corner::DRB,
    Corner::DFR,
    Corner::DLF,
    Corner::ULB,
    Corner::DBL,
];
const _CO_B: [usize; 8] = [0, 0, 1, 2, 0, 0, 2, 1];
const _EP_B: [Edge; 12] = [
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
const _EP__B: [usize; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

/// Move Array

const MOVEMENTS: [CubieCube; 6] = [
    CubieCube::new_from_val(_CP_U, _CO_U, _EP_U, _EO_U),
    CubieCube::new_from_val(_CP_R, _CO_R, _EP_R, _EO_R),
    CubieCube::new_from_val(_CP_F, _CO_F, _EP_F, _EO_F),
    CubieCube::new_from_val(_CP_D, _CO_D, _EP_D, _EO_D),
    CubieCube::new_from_val(_CP_L, _CO_L, _EP_L, _EO_L),
    CubieCube::new_from_val(_CP_B, _CO_B, _EP_B, _EO_B),
];
