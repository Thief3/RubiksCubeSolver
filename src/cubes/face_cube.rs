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

use crate::defs::facelets::{ GetChar };

pub struct FaceCube {
    f: [Facelets; 54]
}

impl FaceCube {
    /// Make a new FaceCube from a Vec<char>
    pub fn new(cube_array: Vec<char>) -> FaceCube{
        let mut fc = FaceCube{ f: [Facelets::U; 54] };
        
        if cube_array.len > 54 {
            panic!("You're passing in too large a cube!!");
        }
        else if cube_array.len < 54 {
            panic!("You're passing in too small a cube!!");
        }

        for i in 0..cube_array.len() {
            fc.f[i] = cube_array[i].get_facelets();
        }

        fc
    }

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

        FaceCube::new(a.join(""))
    }

    /// Make a new default state FaceCube.
    pub fn reset() -> FaceCube{
        FaceCube::new("UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB".chars().collect)
    }

    pub fn to_string(&self) -> String{
        self.f.into_iter()
            .map(|x| x.get_char())
            .collect()
            .join()
    }



}
