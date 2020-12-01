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
pub struct CoordCube {
    // Phase One
    twist: usize,
    flip: usize,
    udslice: usize,

    // Phase Two
    edge4: usize,
    edge8: usize,
    corner: usize,
    tables: Tables
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
    pub fn movement(&mut self, m: Moves){
        let m_as_u = m as usize;

        self.twist   = self.tables.twist_move[self.twist][m_as_u] as usize;
        self.flip    = self.tables.flip_move[self.flip][m_as_u] as usize;
        self.udslice = self.tables.udslice_move[self.udslice][m_as_u] as usize;
        self.edge4   = self.tables.edge4_move[self.edge4][m_as_u] as usize;
        self.edge8   = self.tables.edge8_move[self.edge8][m_as_u] as usize;
        self.corner  = self.tables.corner_move[self.corner][m_as_u] as usize;
    }
}

pub enum Moves {
    U1 = 0,
    U2, U3,
    R1, R2, R3,
    F1, F2, F3,
    D1, D2, D3,
    L1, L2, L3,
    B1, B2, B3
}
