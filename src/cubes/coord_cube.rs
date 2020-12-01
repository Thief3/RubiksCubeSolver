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
//! Module for coord level cube.

use crate::prunning::Tables;
use super::cubie_cube::CubieCube;

/// Coordinate Representation of a cube. Updates coordinates using a pre-computed
/// move table.
#[derive(Clone)]
pub struct CoordCube {
    // Phase One
    pub twist: usize,
    pub flip: usize,
    pub udslice: usize,

    // Phase Two
    pub edge4: usize,
    pub edge8: usize,
    pub corner: usize,
    pub tables: Tables
}

impl CoordCube{

    /// Create a CoordCube from a CubieCube struct.
    pub fn from_cubie_cube(cc: CubieCube) -> CoordCube {
        CoordCube{
            twist: cc.twist(),
            flip: cc.flip(),
            udslice: cc.udslice(),
            edge4: cc.edge4(),
            edge8: cc.edge8(),
            corner: cc.corner(),
            tables: Tables::load_tables(),
        }
    }

    /// A Move method to update the coordinates in the cubie cube from tables.
    pub fn movement(&mut self, m: Moves) -> bool{
        let m_as_u = m as usize;

        if  self.tables.twist_move[self.twist][m_as_u] >= 0
            && self.tables.flip_move[self.flip][m_as_u] >=0
            && self.tables.udslice_move[self.udslice][m_as_u] >= 0
            && self.tables.edge4_move[self.edge4][m_as_u] >= 0
            && self.tables.edge8_move[self.edge8][m_as_u] >= 0
            &&  self.tables.corner_move[self.corner][m_as_u] >= 0 {
                self.twist   = self.tables.twist_move[self.twist][m_as_u] as usize;
                self.flip    = self.tables.flip_move[self.flip][m_as_u] as usize;
                self.udslice = self.tables.udslice_move[self.udslice][m_as_u] as usize;
                self.edge4   = self.tables.edge4_move[self.edge4][m_as_u] as usize;
                self.edge8   = self.tables.edge8_move[self.edge8][m_as_u] as usize;
                self.corner  = self.tables.corner_move[self.corner][m_as_u] as usize;
                return true;
            }

        false
    }
}

#[derive(Copy, Clone)]
pub enum Moves {
    U1 = 0,
    U2, U3,
    R1, R2, R3,
    F1, F2, F3,
    D1, D2, D3,
    L1, L2, L3,
    B1, B2, B3
}

pub const MOVE_LIST: [Moves; 18] = [
    Moves::U1, Moves::U2, Moves::U3,
    Moves::R1, Moves::R2, Moves::R3,
    Moves::F1, Moves::F2, Moves::F3,
    Moves::D1, Moves::D2, Moves::D3,
    Moves::L1, Moves::L2, Moves::L3,
    Moves::B1, Moves::B2, Moves::B3
];
