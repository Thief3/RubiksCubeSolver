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
//! Module that deals with the solving of the rubiks cube. This is done in two
//! parts, each focusing on a different mathematical group to solve. Both phases
//! use the same implamentation of IDA*, with different depths and goals.

use prunning;
use std::cmp;
use crate::cubes::coord_cube::{ CoordCube, Moves };
use crate::cubes::face_cube::FaceCube;

struct Solver {
    cc: CoordCube
}

impl Solver{
    pub fn solve(cc: CoordCube, max_depth: usize) -> Vec<Moves>{
        let moves: Vec<Moves> = Vec::new();
        
        for depth in 0..max_depth {
            
        }
        
        moves
    }
    
    pub fn phase_one_cost(self) -> usize{
        std::cmp::max(
            self.cc.tables.udslice_twist_prune.get(
                self.cc.udslice,
                self.cc.twist),
            self.cc.tables.udslice_flip_prune.get(
                self.cc.udslice,
                self.cc.flip)
        ) as usize
    }

    pub fn phase_two_cost(self) -> usize{
        std::cmp::max(
            self.cc.tables.edge4_corner_prune.get(
                self.cc.edge4,
                self.cc.corner),
            self.cc.tables.edge4_edge8_prune.get(
                self.cc.edge4,
                self.cc.edge8)
        ) as usize
    }
}


/// A pattern matching function which dictates what `Moves` are mathematically
/// equal according to our group theory definitions of the cube.
///
/// # Parameters
/// * `movement` - A `Moves` to find the opposite equal of.
/// # Returns
/// * `Moves` - The mathematical equal of `movement`.
fn opposite_move(movement: Moves) -> Moves {
    let a = match movement {
        Moves::F1 => Moves::B1,
        Moves::F2 => Moves::B2,
        Moves::F3 => Moves::B3,
        Moves::B1 => Moves::F1,
        Moves::B2 => Moves::F2,
        Moves::B3 => Moves::F3,
        Moves::U1 => Moves::D1,
        Moves::U2 => Moves::D2,
        Moves::U3 => Moves::D3,
        Moves::D1 => Moves::U1,
        Moves::D2 => Moves::U2,
        Moves::D3 => Moves::U3,
        Moves::L1 => Moves::R1,
        Moves::L2 => Moves::R2,
        Moves::L3 => Moves::R3,
        Moves::R1 => Moves::L1,
        Moves::R2 => Moves::L2,
        Moves::R3 => Moves::L3
    };
    a
}

/// A pattern matching function that dictates which `Moves` should not follow
/// another as they essentially repetitions.
///
/// # Parameters
/// * `movement` - The `Moves` of which to find the matching `Moves`.
/// # Returns
/// * `Moves` - The `Moves` the shouldn't follow `movement`
fn cannot_follow(movement: Moves) -> Moves {
    let a = match movement {
        Moves::F1 => Moves::F2,
        Moves::F2 => Moves::F1,
        Moves::F3 => Moves::F2,
        Moves::B1 => Moves::B2,
        Moves::B2 => Moves::B1,
        Moves::B3 => Moves::B2,
        Moves::U1 => Moves::U2,
        Moves::U2 => Moves::U1,
        Moves::U3 => Moves::U2,
        Moves::D1 => Moves::D2,
        Moves::D2 => Moves::D1,
        Moves::D3 => Moves::D2,
        Moves::L1 => Moves::L2,
        Moves::L2 => Moves::L1,
        Moves::L3 => Moves::L2,
        Moves::R1 => Moves::R2,
        Moves::R2 => Moves::R1,
        Moves::R3 => Moves::R2
    };
    a
}

///*****************************************************************************
///* Constant values.
///****************************************************************************

const MAX_PHASE_ONE_DEPTH: usize = 21;//18;
const PHASE_ONE_MOVE_LIST: [Moves; 18] = [
    Moves::F1,
    Moves::F2,
    Moves::F3,
    Moves::B1,
    Moves::B2,
    Moves::B3,
    Moves::U1,
    Moves::U2,
    Moves::U3,
    Moves::D1,
    Moves::D2,
    Moves::D3,
    Moves::L1,
    Moves::L2,
    Moves::L3,
    Moves::R1,
    Moves::R2,
    Moves::R3,
];
const MAX_PHASE_TWO_DEPTH: usize = 10;//12;
const PHASE_TWO_MOVE_LIST: [Moves; 10] = [
    Moves::U1,
    Moves::U2,
    Moves::U3,
    Moves::B2,
    Moves::F2,
    Moves::D1,
    Moves::D2,
    Moves::D3,
    Moves::L2,
    Moves::R2,
];
