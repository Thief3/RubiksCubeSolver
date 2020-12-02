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

use defs::corner_cubies::Corner;
use defs::edge_cubies::Edge;

use super::face_cube::FaceCube;
use crate::defs;
use crate::utility;
use cubes::{TWIST, FLIP, UDSLICE, EDGE4, EDGE8, CORNER, r_cast, emod};
use super::coord_cube::Moves;

/// Struct to represent a Rubiks cube at a corner and edge level.
#[derive(Copy, Clone)]
pub struct CubieCube {
    pub corner_permutation: [Corner; 8],
    pub corner_orientation: [usize; 8],
    
    pub edge_permutation: [Edge; 12],
    pub edge_orientation: [usize; 12],

    //pub moves: Vec<usize>,
}


impl CubieCube {
    /// Creates a new default, solved, Cube.
    pub fn reset() -> CubieCube{
        CubieCube{
            corner_permutation: defs::facelets::CORNER_LIST,
            corner_orientation: [0; 8],
            edge_permutation: defs::facelets::EDGE_LIST,
            edge_orientation: [0; 12],

            //moves: vec![],
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

            //moves: vec![],
        }
    }

    /// Turns this CubieCube into a FaceCube
    pub fn to_face_cube(self) -> FaceCube{
        let mut fc = FaceCube::reset();
        for i in 0..8 {
            let j = self.corner_permutation[i];
            let ori = self.corner_orientation[i];

            for k in 0..3{
                fc.f[defs::facelets::CORNER_INDEXES[i][emod((k + ori) as isize, 3) as usize] as usize] =
                    defs::facelets::CORNER_COLOR[j as usize][k];
            }
        }
        for i in 0..12 {
            let j = self.edge_permutation[i];
            let ori = self.edge_orientation[i];

            for k in 0..2 {
                fc.f[defs::facelets::EDGE_INDEXES[i][emod((k + ori) as isize, 2) as usize]] =
                    defs::facelets::EDGE_COLOR[j as usize][k];
            }
        }

        fc
    }

    /// Computes the permuation and orientation of the corners after applying a
    /// permutation to the current cube.
    #[allow(dead_code)]
    pub fn corner_multiply(&mut self, a: CubieCube){
        let mut cp: [Corner; 8] = [Corner::URF; 8];
        let mut co: [usize; 8] = [0; 8];

        for i in 0..8{
            cp[i] = self.corner_permutation[a.corner_permutation[i] as usize];
            co[i] = self.corner_orientation[a.corner_orientation[i]] + emod(a.corner_orientation[i] as isize, 2) as usize;
        }

        self.corner_permutation = cp;
        self.corner_orientation = co;
    }

    /// Computes the permuation and orientation of the edges after applying a
    /// permutation to the current cube.
    #[allow(dead_code)]
    pub fn edge_multiply(&mut self, a: CubieCube){
        let mut ep: [Edge; 12] = [Edge::UR; 12];
        let mut eo: [usize; 12] = [0; 12];

        for i in 0..12{
            ep[i] = self.edge_permutation[a.edge_permutation[i] as usize];
            eo[i] = self.edge_orientation[a.edge_orientation[i]] + emod(a.edge_orientation[i] as isize, 1) as usize;
        }

        self.edge_permutation = ep;
        self.edge_orientation = eo;
    }

    /// Computes both the edge and corner permutations and orientations.
    #[allow(dead_code)]
    pub fn multiply(&mut self, a: CubieCube){
        self.corner_multiply(a);
        self.edge_multiply(a);
    }

    /// Move helper function that takes a Move type.
    pub fn movement(&mut self, movement: Moves){
        let p = movement as usize % 3;
        let m = (movement as usize - p)/3;
        for _i in 0..p{
            self.multiply(MOVEMENTS[m]);
        }

        //self.moves.push(movement as usize);
    }

    /// Move helper function
    pub fn movement_f(&mut self, movement: defs::facelets::Facelets){
        self.multiply(MOVEMENTS[movement as usize]);
        //self.moves.push(movement as usize);
    }

    /// Move helper function that takes usize
    pub fn movement_u(&mut self, movement: usize){
        if movement < MOVEMENTS.len() {
            self.multiply(MOVEMENTS[movement]);
        }
        else{
            panic!("Move {} doesn't exist!!", movement);
        }

        //self.moves.push(movement as usize);
    }

    /// Inverses the current cube and returns.
    #[allow(dead_code)]
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
                println!("ori: {}, ori set: {}, ori_modded", (- (ori as isize)), emod((- (ori as isize)), 3));
                cc.corner_orientation[i] = emod(- (ori as isize),3) as usize;
            }
        }

        println!("edge perm, ori {:?}: {:?}", cc.edge_permutation, cc.edge_orientation);
        println!("Corner, ori {:?}: {:?}", cc.corner_permutation, cc.corner_orientation);
        cc
    }

    /// Checks the cube can be solved.
    pub fn can_solve(self) -> usize{
        let mut total: usize = 0;
        let mut edge_count: [usize; 12] = [0; 12];
        let mut corner_count: [usize; 8] = [0; 8];

        //print!("Edge Permutation: {:?}\n", self.edge_permutation);
        // Not all edges exist.
        for e in 0..12{
            edge_count[self.edge_permutation[e] as usize] += 1;
        }
        for i in 0..12 {
            //print!("Edge: {}\n", edge_count[i]);
            if edge_count[i] != 1{
                //print!("Edge_count: {:?}\n", edge_count);
                return 2;
            }
        }

        // Flip error: one edge should be flipped.
        for i in 0..12{
            total = total + self.edge_orientation[i];
        }
        if emod(total as isize, 2) != 0{
            return 3;
        }
        
        // Not all corners exist.
        for i in 0..8{
            corner_count[self.corner_permutation[i] as usize] += 1;
        }
        for i in 0..8{
            if corner_count[i] != 1{
                return 4
            }
        }
        
        // Twist error: A corner must be twisted.
        total = 0;
        for i in 0..8{
            total = total + self.corner_orientation[i];
        }
        if emod(total as isize, 3) != 0{
            return 5;
        }
        if self.edge_parity() != self.corner_parity(){
            return 6;
        }
        
        // Parity error: Two corners or edges have to be exchanged.

        // Success
        0
    }

    /// Outputs strings for can_solve
    pub fn can_solve_matcher(self) -> (String, bool) {
        match self.can_solve(){
            0 => return ("Attempting solve...".to_string(), true),
            1 => return ("Each colour should appear exactly 9 time.".to_string(), false),
            2 => return ("Not all edges exist.".to_string(), false),
            3 => return ("One edge should be flipped.".to_string(), false),
            4 => return ("Not all corners exist.".to_string(), false),
            5 => return ("One corner should be twisted.".to_string(), false),
            6 => return ("Two corners or edges should be exchanged.".to_string(), false),
            _ => panic!("Hooooooowwww did you get here!!????")
        }
    }

    /// Property Methods

    /// Corner Parity of cube. This must equal the edge parity for the cube to be solveable.
    #[allow(dead_code)]
    pub fn corner_parity(self) -> usize{
        let mut s = 0;
        for i in (0..8).rev(){
            for j in (0..i).rev(){
                if self.corner_permutation[j] > self.corner_permutation[i]{
                    s = s + 1;
                }
            }
        }

        emod(s, 2) as usize
    }

    /// Edge Parity of a cube. This must equal the corner parity of the cube to be aolveable.
    #[allow(dead_code)]
    pub fn edge_parity(self) -> usize{
        let mut s = 0;
        for i in (0..12).rev() {
            for j in (0..i).rev() {
                if self.edge_permutation[j] > self.edge_permutation[i]{
                    s = s + 1;
                }
            }
        }

        emod(s, 2) as usize
    }

    // Phase One Coordinates.

    /// Get Twist property, the coordinate representing the corner orientation.
    /// Between 0 and 3^7 - 1.
    #[allow(dead_code)]
    pub fn twist(self) -> usize{
        let mut s = 0;
        for corner in 0..7 {
            s = s + self.corner_orientation[corner] * (3_i32.pow(6 - corner as u32) as usize);
        }

        s as usize
    }

    /// Takes a twist value, and sets the corner orientation to the matching array.
    #[allow(dead_code)]
    pub fn set_twist(&mut self, twist: usize){
        if twist >= 3_usize.pow(7) {
            panic!("Twist: {}, is out of range. Must be between 0 and 2186.", twist);
        }
        
        let mut t = twist;
        let mut total = 0;

        for i in 0..7{
            let x = emod(t as isize, 3) as usize;
            self.corner_orientation[6 - i] = x;
            total = total + x;
            t = (t as f64 / 3.0).floor() as usize;
        }
        self.corner_orientation[7] = emod(- (total as isize), 3) as usize;
    }

    /// Get Flip property, the coordinate representing the edge orientation.
    /// Between 0 and 2^11 - 1.
    #[allow(dead_code)]
    pub fn flip(self) -> usize{
        let mut s = 0;
        for edge in 0..12 {
            s = s + self.edge_orientation[edge] * (2_i64.pow(11 - edge as u32) as usize);
        }
        s as usize
    }

    /// Takes a Flip property, and sets the edge orientation to the matching array.
    #[allow(dead_code)]
    pub fn set_flip(&mut self, flip: usize){
        if flip >= 2_usize.pow(11){
            panic!("Flip: {}, is out of range. It must be between 0 and 2047.", flip);
        }

        let mut f = flip;
        let mut total = 0;

        for i in 0..11 {
            let x = emod(flip as isize, 2) as usize;
            self.edge_orientation[10 - i] = x;
            total = total + x;
            f = (f as f64 / 2.0).floor() as usize;
        }
        self.edge_orientation[11] = emod(- (total as isize), 2) as usize;
    }

    /// Computes the udslice coordinate. This coordinate represents the position
    /// of the edges FR, FL, Bl, BR.
    /// Phase two can only start when this value is 0, representing that these
    /// edges are in the middle layer.
    /// UDslice is a value between 0 and (12C4) - 1.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

    // Phase Two Coordinates

    /// Edge4 getter. Edge4 is the coordinate that represents the permutation of
    /// the edges FR, FL, BL, BR. (This assumes we are in phase two.)
    /// Edge4 is between 0 and 23.
    #[allow(dead_code)]
    pub fn edge4(self) -> usize{
        let mut out = 0;
        let mut edge4: [Edge; 4] = [Edge::DB; 4];
        for i in 8..12{
            edge4[i - 8] = self.edge_permutation[i];
        }

        for j in (0..4).rev(){
            let mut s = 0;
            for i in 0..j{
                if edge4[i] > edge4[j]{
                    s = s + 1;
                }
            }
            out = j * ( out + s);
        }

        out
    }

    /// Edge4 setter. Takes in an edge4 value and sets the cube to that
    /// permutation of edges.
    #[allow(dead_code)]
    pub fn set_edge4(&mut self, e: usize){
        if e >= 24{
            panic!("Edge4 {}, is out of range. Ensure it is between 0 and 23(inclusive.)", e);
        }

        let mut edge4 = e;
        let mut slice_edge: Vec<Edge> = vec![Edge::FR, Edge::FL, Edge::BL, Edge::BR];
        let mut cef: [usize; 3] = [0; 3];
        let mut perm: [usize; 4] = [0; 4];
        
        for i in 1..4{
            cef[i - 1] = emod(edge4 as isize, i + 1) as usize;
            edge4 = (edge4 as f64 / (i as f64 + 1.0)).floor() as usize;
        }

        for i in (1..4).rev(){
            perm[i] = slice_edge[i - cef[i - 1]] as usize;
            slice_edge.remove(i - cef[i - 1]);
        }
        perm[0] = slice_edge[0] as usize;

        for i in 8..12{
            self.edge_permutation[i] = defs::facelets::EDGE_LIST[perm[i - 8]];
        }
    }

    /// Edge8 getter. The coordinate representing the permutation of the other 8
    /// edges: UR, UF, UB, DR, DF, DL, DB.
    /// Between 1 and 8! - 1
    #[allow(dead_code)]
    pub fn edge8(self) -> usize{
        let mut edge8 = 0;
        for j in (0..8).rev(){
            let mut s = 0;
            for i in 0..j {
                if self.edge_permutation[i] > self.edge_permutation[j]{
                    s = s + 1;
                }
            }
            edge8 = j * (edge8 + s);
        }

        edge8
    }

    /// Edge8 setter. Sets the order of the edges: UR, UF, UL, UB, DR, DF, DL, DB
    #[allow(dead_code)]
    pub fn set_edge8(&mut self, e: usize){
        if e >= 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1 {
            panic!("Edge8 {}, is out of range. Ensure it is between 0 and 8!.");
        }

        let mut edge8 = e;
        let mut edges: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut cef: [usize; 7] = [0; 7];
        let mut perm: [usize; 8] = [0; 8];

        for i in 0..8{
            cef[i - 1] = emod(edge8 as isize, i + 1) as usize;
            edge8 = (edge8 as f64 / (i as f64+ 1.0)).floor() as usize;
        }

        for i in 1..7{
            perm[i] = edges[i - cef[i - 1]];
            edges.remove(i - cef[i -1]);
        }
        perm[0] = edges[0];
        for i in 0..8{
            self.edge_permutation[i] = defs::facelets::EDGE_LIST[perm[i]];
        }
    }

    /// Corner getter. Gets the coordinate representing the permutation of the
    /// 8 corners: UR, UF, UL, UB, DR, DF, DL, DB.
    /// Value is between 0 and 8! - 1;
    #[allow(dead_code)]    
    pub fn corner(self) -> usize{
        let mut c = 0;
        for j in (1..8).rev(){
            let mut s = 0;
            for i in 0..j{
                if self.corner_permutation[i] > self.corner_permutation[j]{
                    s = s + 1;
                }
            }
            c = j * (c + s);
        }

        c
    }

    /// Corner Setter. Sets the permutation of the corners according to the
    /// parameter passed in.
    #[allow(dead_code)]
    pub fn set_corner(&mut self, c: usize){
        if c >= 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1{
            panic!("Corner {}, is out of range. Please ensure it is between 0 and 8!.", c);
        }

        let mut corner = c;
        let mut corners: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut perm: [usize; 8] = [0; 8];
        let mut cef: [usize; 7] = [0; 7];

        for i in 1..8{
            cef[i - 1] = emod(corner as isize, i + 1) as usize;
            corner = (corner as f64 / (i as f64 + 1.0)).floor() as usize;
        }
        for i in (1..7).rev(){
            perm[i] = corners[i - cef[i -1]];
            corners.remove(i - cef[i - 1]);
        }
        perm[0] = corners[0];
        for i in 0..8{
            self.corner_permutation[i] = defs::facelets::CORNER_LIST[perm[i]];
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
        //moves: vec![]
    },
    CubieCube{
        corner_permutation:  _CP_R,
        corner_orientation: _CO_R,
        edge_permutation:   _EP_R,
        edge_orientation:   _EO_R,
        //moves: vec![]
    },
    CubieCube{
        corner_permutation:  _CP_F,
        corner_orientation: _CO_F,
        edge_permutation:   _EP_F,
        edge_orientation:   _EO_F,
        //moves: vec![]
    },
    CubieCube{
        corner_permutation:  _CP_D,
        corner_orientation: _CO_D,
        edge_permutation:   _EP_D,
        edge_orientation:   _EO_D,
        //moves: vec![]
    },
    CubieCube{
        corner_permutation:  _CP_L,
        corner_orientation: _CO_L,
        edge_permutation:   _EP_L,
        edge_orientation:   _EO_L,
        //moves: vec![]
    },
    CubieCube{
        corner_permutation:  _CP_B,
        corner_orientation: _CO_B,
        edge_permutation:   _EP_B,
        edge_orientation:   _EO_B,
        //moves: vec![]
    }
];
