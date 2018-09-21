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
//! Module regulates the face values and handles converting facelet input into
//! cubie form so that we can solve the problem.

use edge_cubies::{Edges};
use corner_cubies::{Corners};
use physical::{Cube};

/// A enum of the different possible face values.
#[derive(Copy, Clone, Debug)]
pub enum Facelets {
    U = 0,
    R,
    F,
    D,
    L,
    B,
}

/// A struct to regulate the facelets of the cube before they are converted into
/// cube form.
///
/// # Parameters
/// * `faclets_first_half` - An array of all the faces on a cube arranged such
///     that the first 9 values represent the upper face. The next 9 the right
///     face, and so on in th order upper, right and front. Within each nine
///     values, the first is the top left, the next middle top and we carry
///     accross and down tothe bottom right.
/// * `facelets_second_half` - As above but for the faces down, left and back
///     The reason for the split is regarding the debug trait not working for
///     "large" arrays.

#[derive (Copy, Clone, Debug)]
pub struct Face {
    facelets_first_half: [Facelets; 27],
    facelets_second_half: [Facelets; 27],
}

impl Face{
    pub fn new() -> Face {
        let face = Face {
        let facelets_vals = [Facelets::U, Facelets::R, Facelets::F, Facelets::D, Facelets::L, Facelets::B,];
        for i in 0..3 {
            for j in 0..9 {
                face.facelets_first_half[i * 9 + j] = facelets_vals[i];
            }
        }
        for i in 3..6 {
            for j in 0..9 {
                face.facelets_second_half[i * 9 + j] = facelets_vals[i];
            }
        }
        }

    }
    
    /// A setter method for the facelets arrays in `Cube`. This allows us to
    /// manage the two halfs of the array as one.
    ///
    /// # Parameters
    /// * `index` - The index of which you wish to change. Between 0 and 53
    /// * `val` - The value you wish to change the specific face to.
    pub fn set_facelets(&mut self, index: usize, val: Facelets){
        if index < 27 && index >= 0 {
            self.facelets_first_half[index] = val;
        }else if index > 27 && index <= 53 {

           self.facelets_second_half[53 - index] = val;
        }else {
            panic!("set_facelets: Outside the index range for facelets. Keep index within 0 and 53. Index found: {}", index);
        }
    }

    /// A getter method fo rthe facelet arrays in `Cube`. This allows us to
    /// manage the two halfs of the array as one.
    ///
    /// # Parameters
    /// * `index` - The index of the facelets arrays you wish to access, must
    ///    be between 0 and 53 or the function will panic.
    pub fn get_facelets(& self, index:usize) -> Facelets{
        if index < 27 && index >= 0 {
            return self.facelets_first_half[index]
        }else if index > 27 && index <= 53 {
            return self.facelets_second_half[53 - index]
        }else {
            panic!("get_facelets: Outside the index range for facelets. Keep index within 0 and 53. Index found: {}", index);
        }
    }

    /// A method that checks that the current face is solveable.
    /// # Returns
    /// * `bool` - True if solveable, false is not.
    pub fn check_if_can_be_solved(&self) -> bool{
        
    }

    /// A method to turn a face into a cube.
    /// # Returns
    /// * `Cube` - A `Cube` with values homomorphic to this face.
    pub fn turn_into_cube(&self) -> Cube{
        
    }
}

/// ****************************************************************************
/// * Definitions
/// ****************************************************************************

/// A pattern matching method that takes an `Edges` and returns the two
/// `Face` index positions that belong to that edge.
///
/// # Parameters
/// * `e` - An edge to get the corresponding `Face` index positions.
/// # Returns
/// * `[usize; 2]` - The two index positions connected to `e`.
pub fn edge_indexes (e: Edges) -> [usize; 2]{
    
}

/// A pattern matching method that takes an `Edges` and returns the two
/// `Facelets` that belong to that edge.
///
/// # Parameters
/// * `e` - An edge to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 2]` - The two `Facelets` connected to `e`.
pub fn edge_colours (e: Edges) -> [Facelets; 2]{
    match ref e {
        Edges::UR => [Facelets::U,Facelets::R,],
        Edges::UF => [Facelets::U,Facelets::F,],
        Edges::UL => [Facelets::U,Facelets::L,],
        Edges::UB => [Facelets::U,Facelets::B,],
        Edges::DR => [Facelets::D,Facelets::R,],
        Edges::DF => [Facelets::D,Facelets::F,],
        Edges::DL => [Facelets::D,Facelets::L,],
        Edges::DB => [Facelets::D,Facelets::B,],
        Edges::FR => [Facelets::F,Facelets::R,],
        Edges::FL => [Facelets::F,Facelets::L,],
        Edges::BL => [Facelets::B,Facelets::L,],
        Edges::BR => [Facelets::B,Facelets::R,],
    }
}

/// A pattern matching method that takes a `Corners` and returns the three
/// `Face` index positions that belong to that corner.
///
/// # Parameters
/// * `c` - A corner to get the corresponding `Face` index positions.
/// # Returns
/// * `[usize; 3]` - The three index positions connected to `c`.
pub fn corner_indexes (c: Corners) -> [usize; 3]{
    
}

/// A pattern matching method that takes a `Corners` and returns the three
/// `Facelets` that belong to that corner.
///
/// # Parameters
/// * `c` - An corner to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 3]` - The two `Facelets` connected to `c`.
pub fn corner_colours (c: Corners) - [Facelets; 3]{
    Corners::URF => [Corners::U, Corners::R, Corners::F,],
    Corners::UFL => [Corners::U, Corners::F, Corners::L,],
    Corners::ULB => [Corners::U, Corners::L, Corners::B,],
    Corners::UBR => [Corners::U, Corners::B, Corners::R,],
    Corners::DFR => [Corners::D, Corners::F, Corners::R,],
    Corners::DLF => [Corners::D, Corners::L, Corners::F,],
    Corners::DBL => [Corners::D, Corners::B, Corners::L,],
    Corners::DRB => [Corners::D, Corners::R, Corners::B,],
}
