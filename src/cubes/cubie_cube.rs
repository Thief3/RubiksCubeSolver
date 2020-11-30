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
use super::face_cube::FaceCube;
use crate::defs;
use crate::utility;

/// Struct to represent a Rubiks cube at a corner and edge level.
#[derive(Copy, Clone)]
pub struct CubieCube {
    pub corner_permutation: [Corner; 8],
    pub corner_orientation: [usize; 8],
    
    pub edge_permutation: [Edge; 12],
    pub edge_orientation: [usize; 12],
}

impl CubieCube {
    /// Creates a new default, solved, Cube.
    pub fn reset() -> CubieCube{
        CubieCube{
            corner_permutation: defs::facelets::CORNER_LIST,
            corner_orientation: [0; 8],
            edge_permutation: defs::facelets::EDGE_LIST,
            edge_orientation: [0; 12],
        }
    }

    /// Helper function for creating a new CubieCube with prechosen values.
    #[allow(dead_code)]
    pub fn new(
        cp: [Corner; 8],
        co: [usize; 8],
        ep: [Edge; 12],
        eo: [usize; 12]) -> CubieCube{
        CubieCube{
            corner_permutation: cp,
            corner_orientation: co,
            edge_permutation: ep,
            edge_orientation: eo,
        }
    }

    /// Turns this CubieCube into a FaceCube
    pub fn to_facecube(self) -> FaceCube{
        let mut fc = FaceCube::reset();
        for i in 0..8 {
            let j = self.corner_permutation[i];
            let ori = self.corner_orientation[i];

            for k in 0..3{
                fc.f[defs::facelets::CORNER_INDEXES[i][(k + ori) % 3] as usize] =
                    defs::facelets::CORNER_COLOR[j as usize][k];
            }
        }
        for i in 0..12 {
            let j = self.edge_permutation[i];
            let ori = self.edge_orientation[i];

            for k in 0..2 {
                fc.f[defs::facelets::EDGE_INDEXES[i][(k + ori) % 2]] =
                    defs::facelets::EDGE_COLOR[j as usize][k];
            }
        }

        fc
    }

    /// Computes the permuation and orientation of the corners after applying a
    /// permutation to the current cube.
    pub fn corner_multiply(&mut self, a: CubieCube){
        let mut cp: [Corner; 8] = [Corner::URF; 8];
        let mut co: [usize; 8] = [0; 8];

        for i in 0..8{
            cp[i] = self.corner_permutation[a.corner_permutation[i] as usize];
            co[i] = self.corner_orientation[a.corner_orientation[i]] + a.corner_orientation[i] % 3;
        }

        self.corner_permutation = cp;
        self.corner_orientation = co;
    }

    /// Computes the permuation and orientation of the edges after applying a
    /// permutation to the current cube.
    pub fn edge_multiply(&mut self, a: CubieCube){
        let mut ep: [Edge; 12] = [Edge::UR; 12];
        let mut eo: [usize; 12] = [0; 12];

        for i in 0..12{
            ep[i] = self.edge_permutation[a.edge_permutation[i] as usize];
            eo[i] = self.edge_orientation[a.edge_orientation[i]] + a.edge_orientation[i] % 3;
        }

        self.edge_permutation = ep;
        self.edge_orientation = eo;
    }

    /// Computes both the edge and corner permutations and orientations.
    pub fn multiply(&mut self, a: CubieCube){
        self.corner_multiply(a);
        self.edge_multiply(a);
    }

    /// Move helper function
    pub fn movement(&mut self, movement: defs::facelets::Facelets){
        self.multiply(MOVEMENTS[movement as usize]);
    }

    /// Move helper function that takes usize
    pub fn movement_u(&mut self, movement: usize){
        if movement < MOVEMENTS.len() {
            self.multiply(MOVEMENTS[movement]);
        }
        else{
            panic!("Move {} doesn't exist!!", movement);
        }
    }

    /// Inverses the current cube and returns.
    pub fn inverse_cubiecube(self) -> CubieCube {
        let mut cc: CubieCube = self;

        for i in 0..12 {
            cc.edge_permutation[self.edge_permutation[i] as usize] = defs::facelets::EDGE_LIST[i];
            if i < 8 {
                cc.corner_permutation[self.corner_permutation[i] as usize] = defs::facelets::CORNER_LIST[i];
            }
        }

        for i in 0..12 {
            cc.edge_orientation[i] = self.edge_orientation[cc.edge_permutation[i] as usize];
            if i < 8 {
                let ori = self.corner_orientation[cc.corner_permutation[i] as usize];
                cc.corner_orientation[i] = (- (ori as isize) % 3) as usize;
            }
        }

        cc
    }

    /// Property Methods

    /// Corner Parity of cube. This must equal the edge parity for the cube to be solveable.
    pub fn corner_parity(self) -> usize{
        let mut s = 0;
        for i in (0..8).rev(){
            for j in (0..i).rev(){
                if self.corner_permutation[j] > self.corner_permutation[i]{
                    s = s + 1;
                }
            }
        }

        s % 2
    }

    /// Edge Parity of a cube. This must equal the corner parity of the cube to be aolveable.
    pub fn edge_parity(self) -> usize{
        let mut s = 0;
        for i in (0..12).rev() {
            for j in (0..i).rev() {
                if self.edge_permutation[j] > self.edge_permutation[i]{
                    s = s + 1;
                }
            }
        }

        s % 2
    }

    // Phase One Coordinates.

    /// Get Twist property, the coordinate representing the corner orientation.
    /// Between 0 and 3^7 - 1.
    pub fn twist(self) -> usize{
        let mut s = 0;
        for corner in 0..7 {
            s = s + self.corner_orientation[corner] * (3_i32.pow(6 - corner as u32) as usize);
        }

        s as usize
    }

    /// Takes a twist value, and sets the corner orientation to the matching array.
    pub fn set_twist(&mut self, twist: usize){
        if twist < 0 || twist >= 3_usize.pow(7) {
            panic!("Twist: {}, is out of range. Must be between 0 and 2186.", twist);
        }
        
        let mut t = twist;
        let mut total = 0;

        for i in 0..7{
            let x = t % 3;
            self.corner_orientation[6 - i] = x;
            total = total + x;
            t = (t as f64 / 3.0).floor() as usize;
        }
        self.corner_orientation[7] = (- (total as isize) % 3) as usize;
    }

    /// Get Flip property, the coordinate representing the edge orientation.
    /// Between 0 and 2^11 - 1.
    pub fn flip(self) -> usize{
        let mut s = 0;
        for edge in 0..12 {
            s = s + self.edge_orientation[edge] * (2_i64.pow(11 - edge as u32) as usize);
        }
        s as usize
    }

    /// Takes a Flip property, and sets the edge orientation to the matching array.
    pub fn set_flip(&mut self, flip: usize){
        if flip < 0 || flip >= 2_usize.pow(11){
            panic!("Flip: {}, is out of range. It must be between 0 and 2047.", flip);
        }

        let mut f = flip;
        let mut total = 0;

        for i in 0..11 {
            let x = flip % 2;
            self.edge_orientation[10 - i] = x;
            total = total + x;
            f = (f as f64 / 2.0).floor() as usize;
        }
        self.edge_orientation[11] = (- (total as isize) % 2 ) as usize;
    }

    /// Computes the udslice coordinate. This coordinate represents the position
    /// of the edges FR, FL, Bl, BR.
    /// Phase two can only start when this value is 0, representing that these
    /// edges are in the middle layer.
    /// UDslice is a value between 0 and (12C4) - 1.
    pub fn udslice(self) -> usize{
        let mut udslice = 0;
        let mut seen = 0;

        for j in 0..12{
            if (self.edge_permutation[j] as usize) >= 8
                && (self.edge_permutation[j] as usize) < 12 {
                    seen = seen + 1;
                }
            else if seen >= 1{
                udslice = udslice + utility::binomial(j as i64, (seen - 1) as i64);
            }
        }

        udslice as usize
    }

    /// Sets the UDslice of the cube. It takes in a UDslice and sets the positions
    /// for FR, FL, BL and BR.
    pub fn set_udslice(&mut self, u: usize){
        if (u as i64) >= utility::binomial(12, 4){
            panic!("UDSlice {}, is out of range. Make sure it is between 0 and 494", u);
        }

        let udslice_edge: [Edge; 4] = [Edge::FR, Edge::FL, Edge::BL, Edge::BR];
        let other_edge: [Edge; 8] = [
            Edge::UR,
            Edge::UF,
            Edge::UL,
            Edge::UB,
            Edge::DR,
            Edge::DF,
            Edge::DL,
            Edge::DB,
        ];
        let mut seen = 3;
        let mut udslice = u;
        
        for i in 0..12{
            self.edge_permutation[i] = Edge::DB;
        }

        for j in (0..12).rev(){
            if udslice as i64 - utility::binomial(j as i64, seen as i64) < 0{
                self.edge_permutation[j] = udslice_edge[seen];
                seen = seen - 1;
            }
            else{
                udslice = (udslice as i64- utility::binomial(j as i64, seen as i64)) as usize;
            }
        }

        let mut x = 0;
        for j in 0..12{
            if self.edge_permutation[j] == Edge::DB {
                self.edge_permutation[j] = other_edge[x];
                x = x + 1;
            }
        }
    }

    
}


// Definitions for moves

/// Upper Moves
const _CP_U: [Corner; 8] = [
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
const _EO_U: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
const _EO_R: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
const _EO_F: [usize; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

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
const _EO_D: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
const _EO_L: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
const _EO_B: [usize; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

// Move Array
const MOVEMENTS: [CubieCube; 6] = [
    CubieCube{
        corner_permutation:  _CP_U,
        corner_orientation: _CO_U,
        edge_permutation:   _EP_U,
        edge_orientation:   _EO_U,
    },
    CubieCube{
        corner_permutation:  _CP_R,
        corner_orientation: _CO_R,
        edge_permutation:   _EP_R,
        edge_orientation:   _EO_R,
    },
    CubieCube{
        corner_permutation:  _CP_F,
        corner_orientation: _CO_F,
        edge_permutation:   _EP_F,
        edge_orientation:   _EO_F,
    },
    CubieCube{
        corner_permutation:  _CP_D,
        corner_orientation: _CO_D,
        edge_permutation:   _EP_D,
        edge_orientation:   _EO_D,
    },
    CubieCube{
        corner_permutation:  _CP_L,
        corner_orientation: _CO_L,
        edge_permutation:   _EP_L,
        edge_orientation:   _EO_L,
    },
    CubieCube{
        corner_permutation:  _CP_B,
        corner_orientation: _CO_B,
        edge_permutation:   _EP_B,
        edge_orientation:   _EO_B,
    }
];
