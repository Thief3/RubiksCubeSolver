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
//! A module for the physical representation of a Rubiks cube.
//!
//! Deals with all the moves a cube has, as well as insuring the
//! transformation are in a group/coordinate style that best works with
//! the two-phase algorithm.

use utility;
pub mod corner_cubies;
pub mod edge_cubies;

/// The main struct of the program.
///
/// Defines a representation of a physical rubiks cube using a group theory
/// style notation. This is such that it will be best optimised when used with
/// the two-phase algorithm as designed by [Kociemba](http://kociemba.org).
///
/// # Variables
///
/// * `corner_orientation` - A value between 0 and 2186, representing the
///     orientation of the corners overall.
/// * `edge_orientation` - A value between 0 and 2047, representing the
///     orientation of the edges overall.
/// * `corner_permutation` - A value between 0 and 40319, representing the
///     permutation of the cubes corners.
/// * `phase_two_edge_permutaion` - A value between 0 and 40320, but between
///     0 and 24 for a G1 state `Cube` the permutation of the cubes edges, only
///     valid in phase 2..
/// * `corner_parity` - The parity of the corner permutation.
/// * `edge_parity` - The parity of the edge permutation.
/// * `ud_slice` - A value between 0 and 494, representing the front UD
///     slice edges.
/// * `corners` - An array of the 8 `CornerCubies`.
/// * `edges` - An array of the 12 `EdgeCubies`.
#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub corners: [corner_cubies::CornerCubie; 8],
    pub edges: [edge_cubies::EdgeCubie; 12],
}

impl Cube {
    /// Creates a new `Cube` object with all values set at start positions.
    /// # Return
    /// * `Cube`
    pub fn new() -> Cube {
        let mut new_cube = Cube {
            corners: [
                corner_cubies::CornerCubie::new(corner_cubies::Corner::URF),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::UFL),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::ULB),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::UBR),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DFR),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DLF),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DBL),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DRB),
            ],
            edges: [
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::UR),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::UF),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::UL),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::UB),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::DR),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::DF),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::DL),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::DB),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::FR),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::FL),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::BL),
                edge_cubies::EdgeCubie::new(edge_cubies::Edge::BR),
            ],
        };
        new_cube
    }

    /// Calculates the corner orientation.
    ///
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner orientation of the whole cube.  Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn corner_orientation(&self) -> usize{
        let mut s = 0;
        for corner in 0..7 {
            s = s + self.corners[corner].orientation * 3_i32.pow(6 - corner as u32);
        }
        s as usize
    }

    /// Calculates the corner permutation.
    ///
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner permutation of the whole cube. Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn corner_permutation(&self) -> usize{
        let mut s = 0;
        for i in 1..8 {
            let mut diff = 0_i32;
            for j in 0..i {
                if self.corners[j].coordinate as i32 > self.corners[i].coordinate as i32 {
                    diff = diff + 1;
                }
            }
            s = s + diff * utility::factorial(i as i64) as i32;
        }
        s as usize
    }

    /// Calculates the edge orientation.
    ///
    /// Should be called after every movement. Calculates a binary value used
    /// to represent the edge orientation of the whole cube.  Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn edge_orientation(&self) -> usize{
        let mut s = 0;
        for edge in 0..12 {
            s = s + self.edges[edge].orientation * 2_i32.pow(11 - edge as u32);
        }
        s as usize
    }

    /// Calculates the edge permutation.
    ///
    /// Should be called after every movement. Calculates a binary value used
    /// to represent the corner permutation of the whole cube. Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn edge_permutation(&self) -> usize{
        let mut s = 0;
        for i in 0..12 {
            s = s + self.edges[i].orientation * 2_i32.pow((11 - i) as u32)
        }
        s as usize
    }
    
    /// Calculates the UD Slice.
    ///

    /// This is best explained at the link. Essentially we take the positions
    /// of the UD slices and and any edges in between the positions (and
    /// position "12") are taken. These positions are then used to calculate
    /// a binomial coefficent, with k comibinations of:
    ///     -1 + the number of UD Slices to the left(smaller) than the current.
    /// Further explanation at (http://kociemba.org/math/UDSliceCoord.htm)
    pub fn ud_slice(&self) -> usize{
        // FR, FL, BL, BR are the UD slice edges. This corresponds to the last
        // four values in our edges array, so we take these values and order
        // them for the algorithm.
        0
    }

    /// Calculates the UD sorted slice.
    ///
    /// The permutation and location of the UD-Slice edges.
    pub fn ud_sorted_slice(&self) -> usize{
        let mut x: i32 = 0;
        let mut a = 0;
        let mut edge4: [edge_cubies::Edge; 4] = [
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
        ];

        // range 
        for j in (0..12).rev() {
            if self.edges[j as usize].coordinate == edge_cubies::Edge::FR
                || self.edges[j as usize].coordinate == edge_cubies::Edge::FL
                || self.edges[j as usize].coordinate == edge_cubies::Edge::BL
                || self.edges[j as usize].coordinate == edge_cubies::Edge::BR
            {
                a = a + utility::binomial(11 - j as i64, x as i64 + 1) as i32;
                edge4[3 - x as usize] = self.edges[j as usize].coordinate;
                x = x + 1
            }
        }

        let mut b = 0;
        print!("edge4: {:?}\n {}:{}:{}:{}\n", edge4, edge4[0] as usize, edge4[1] as usize, edge4[2] as usize, edge4[3] as usize);
        for j in (0..3).rev(){
            let mut k = 0;
            while edge4[j] as usize != j + 8 {
                //print!("edge4: {}, j: {}, j + 8: {} \n", edge4[j] as usize, j, j + 8);
                //rotate_left(edge4, 0, j);
                let temp = edge4[0];
                for i in 0..j {
                    edge4[i] = edge4[i + 1];
                }
                edge4[j] = temp;
                //
                k = k + 1;
            }
            b = (j + 1) * b + k;
        }
        
        (24 * a + b as i32) as usize
    }
    //uuuuuuuuubffbrfdbdlbrlfllfbflrdddfflflbblddrdrrlrbrbdr
    /// Calculates the phase two edge permutation.
    ///
    /// Calculates a description of the edge coordinates, but is only valid
    /// in phase two of the algorithm.
    pub fn phase_two_edge_permutation(&self) -> usize{
        0
    }

    /// Calculates the parity of the corner permutation.
    /// Used only for testing if the cube can be solved.
    pub fn corner_parity(&self) -> usize{
        0
    }

    /// Calculates the parity of the edge permutation.
    /// Used only for testing if the cube can be solved.
    pub fn edge_parity(&self) -> usize{
        0
    }
    
    //////////////////////////////////////////////////////////////////////////////
    // Move List
    /////////////////////////////////////////////////////////////////////////////
    
    /// A clockwise front move.
    pub fn f(&mut self) {
        for i in 0..8 {
            self.corners[i].f();
        }
        for i in 0..12 {
            self.edges[i].f();
        }
    }

    /// A clockwise back move.
    pub fn b(&mut self) {
        for i in 0..8 {
            self.corners[i].b();
        }
        for i in 0..12 {
            self.edges[i].b();
        }
    }

    /// A clockwise left move.
    pub fn l(&mut self) {
        for i in 0..8 {
            self.corners[i].l();
        }
        for i in 0..12 {
            self.edges[i].l();
        }
    }

    /// A clockwise right move.
    pub fn r(&mut self) {
        for i in 0..8 {
            self.corners[i].r();
        }
        for i in 0..12 {
            self.edges[i].r();
        }
    }

    /// A clockwise upper move.
    pub fn u(&mut self) {
        for i in 0..8 {
            self.corners[i].u();
        }
        for i in 0..12 {
            self.edges[i].u();
        }
    }

    /// A clockwise down move.
    pub fn d(&mut self) {
        for i in 0..8 {
            self.corners[i].d();
        }
        for i in 0..12 {
            self.edges[i].d();
        }
    }
}
