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
//! Module for Face level cube.

use defs::edge_cubies::Edge;
use defs::corner_cubies::Corner;

use crate::defs::facelets::{
    GetChar, GetFacelets,
    Color, Facelets,
    EDGE_INDEXES, EDGE_LIST, EDGE_COLOR,
    CORNER_INDEXES, CORNER_LIST, CORNER_COLOR,};
use cubes::cubie_cube::CubieCube;

pub struct FaceCube {
    pub f: [Facelets; 54]
}

impl FaceCube {
    /// Make a new FaceCube from a Vec<char>
    pub fn new(cube_array: Vec<char>) -> FaceCube{
        let mut fc = FaceCube{ f: [Facelets::U; 54] };
        
        if cube_array.len() > 54 {
            panic!("You're passing in too large a cube!!");
        }
        else if cube_array.len() < 54 {
            panic!("You're passing in too small a cube!!");
        }

        for i in 0..cube_array.len() {
            fc.f[i] = cube_array[i].get_facelet();
        }

        fc
    }

    /// Make a new FaceCube from an array of Colors.
    /// The order of colors is ULFRBD because of the net, so we shift the squares.
    fn new_from_colors(rubiks: [Color; 54]) -> FaceCube{
        let mut a: [char; 54] = [' '; 54];
        // Remap the way they are in the gui to the old order required for the algo.
        // Upper
        for i in 0..9{
            a[i] = rubiks[i].get_char();
        }
        // Left
        for i in 9..18{
            a[i + 3 * 9] = rubiks[i].get_char();
        }
        // Front
        for i in 18..27{
            a[i] = rubiks[i].get_char();
        }
        // Right
        for i in 27..36{
            a[i - 9 * 2] = rubiks[i].get_char();
        }
        // Back
        for i in 36..45{
            a[i + 9] = rubiks[i].get_char();
        }
        // Down
        for i in 45..54{
            a[i - 9 * 2] = rubiks[i].get_char();
        }

        FaceCube::new(a.iter().cloned().collect::<Vec<char>>())
    }

    /// Make a new default state FaceCube.
    pub fn reset() -> FaceCube{
        FaceCube::new("UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB".chars().collect())
    }

    pub fn to_string(&self) -> String{
        self.f.iter()
            .map(|x| x.get_char())
            .collect::<String>()
    }

    pub fn to_cubie_cube(&self) -> CubieCube {
        let mut cc = CubieCube::reset();

        // Basically this entire algorithm was recreated from
        // https://github.com/hkociemba/RubiksCube-TwophaseSolver/blob/master/face.py
        for i in 0..8{
            let fac = CORNER_INDEXES[i];
            let col1: Facelets;
            let col2: Facelets;
            let mut o: usize = 0;
            for ori in 0..3 {
                if self.f[fac[ori]] == Facelets::U
                    || self.f[fac[ori]] == Facelets::D
                {
                    o = ori;
                    break;
                }
            }
            col1 = self.f[fac[(o + 1) % 3]];
            col2 = self.f[fac[(o + 2) % 3]];
            for c in 0..8 {
                let col = CORNER_COLOR[c as usize];
                if col1 == col[1] && col2 == col[2] {
                    cc.corner_permutation[i] = CORNER_LIST[c];
                    cc.corner_orientation[i] = o;
                    break;
                }
            }
        }
        
        for i in 0..12{
            for e in 0..12 {
                if self.f[EDGE_INDEXES[i][0]] == EDGE_COLOR[e as usize][0]
                    && self.f[EDGE_INDEXES[i][1]] == EDGE_COLOR[e as usize][1]
                {
                    cc.edge_permutation[i] = EDGE_LIST[e];
                    cc.edge_orientation[i] = 0;
                }

                if self.f[EDGE_INDEXES[i][0]] == EDGE_COLOR[e as usize][1]
                    && self.f[EDGE_INDEXES[i][1]] == EDGE_COLOR[e as usize][0]
                {
                    cc.edge_permutation[i] = EDGE_LIST[e];
                    cc.edge_orientation[i] = 1;
                }
            }
        }
        
        cc
    }

}
