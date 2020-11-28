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
    pub corner_orientation: i32,
    pub edge_orientation: i32,
    pub corner_permutation: i32,
    pub phase_two_edge_permutation: i32,
    pub corner_parity: i32,
    pub edge_parity: i32,
    pub ud_slice: i32,
    pub ud_sorted_slice: i32,
    pub corners: [corner_cubies::CornerCubie; 8],
    pub edges: [edge_cubies::EdgeCubie; 12],
}

impl Cube {
    /// Creates a new `Cube` object with all values set at start positions.
    /// # Return
    /// * `Cube`
    pub fn new() -> Cube {
        let mut new_cube = Cube {
            corner_orientation: 0,
            edge_orientation: 0,
            corner_permutation: 0,
            phase_two_edge_permutation: 0,
            corner_parity: 0,
            edge_parity: 0,
            ud_slice: 0,
            ud_sorted_slice: 0,
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
        new_cube.coordinate_adjustments();
        new_cube
    }

    #[allow(dead_code)]
    pub fn calculate_orientations_init(&mut self) {
        for (i, c) in self.corners.iter_mut().enumerate() {
            c.orientation = (((i as i32 - c.coordinate as i32) % 3) + 3) % 3;
        }
        for (i, e) in self.edges.iter_mut().enumerate() {
            e.orientation = (((i as i32 - e.coordinate as i32) % 4) + 4) % 4;
        }
    }

    /// Calculates the corner orientation.
    ///
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner orientation of the whole cube.  Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn calculate_corner_orientation(&mut self) {
        let mut sum = 0;
        for i in 0..7 {
            sum = sum + self.corners[i].orientation * 3_i32.pow((6 - i) as u32)
        }
        self.corner_orientation = sum;
    }

    /// Calculates the corner permutation.
    ///
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner permutation of the whole cube. Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn calculate_corner_permutation(&mut self) {
        let mut sum = 0;
        for i in 1..8 {
            let mut diff = 0_i32;
            for j in 0..i {
                if self.corners[j].coordinate as i32 > self.corners[i].coordinate as i32 {
                    diff = diff + 1;
                }
            }
            sum = sum + diff * utility::factorial(i as i64) as i32;
        }
        self.corner_permutation = sum;
    }

    /// Calculates the edge orientation.
    ///
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the edge orientation of the whole cube.  Further
    /// explanation at (http://kociemba.org/math/coordlevel.htm)
    pub fn calculate_edge_orientation(&mut self) {
        let mut sum = 0;
        for i in 0..12 {
            sum = sum + self.edges[i].orientation * 2_i32.pow((11 - i) as u32)
        }
        self.edge_orientation = sum
    }

    /// Calculates the UD Slice.
    ///
    /// This is best explained at the link. Essentially we take the positions
    /// of the UD slices and and any edges in between the positions (and
    /// position "12") are taken. These positions are then used to calculate
    /// a binomial coefficent, with k comibinations of:
    ///     -1 + the number of UD Slices to the left(smaller) than the current.
    /// Further explanation at (http://kociemba.org/math/UDSliceCoord.htm)
    pub fn calculate_ud_slice(&mut self) {
        // FR, FL, BL, BR are the UD slice edges. This corresponds to the last
        // four values in our edges array, so we take these values and order
        // them for the algorithm.
        let mut sum = 0;
        let values = [
            edge_cubies::Edge::FR,
            edge_cubies::Edge::FL,
            edge_cubies::Edge::BL,
            edge_cubies::Edge::BR,
        ];

        let mut k = -1;
        for i in 0..12 {
            if values.contains(&self.edges[i].coordinate) == true {
                k = k + 1;
            } else if k >= 0 {
                sum = sum + utility::binomial(i as i64, k as i64) as i32;
            };
        }

        self.ud_slice = sum;
    }

    /// Calculates the UD sorted slice.
    ///
    /// The permutation and location of the UD-Slice edges.
    pub fn calculate_ud_sorted_slice(&mut self) {
        /*
        let mut x: i32 = 0;
        let mut a = 0;
        let mut edge4: [edge_cubies::Edge; 4] = [
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
            edge_cubies::Edge::UB,
        ];

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

        for j in (0..4).rev() {
            //println!("{} out of {:?}", j, (0..4).rev());
            let mut k = 0;
            while edge4[j] as i32 != j as i32 + 8 {
                //+8
                //println!("k: {:1}, j: {:2}, edge4: {:?}", k, (j + 8) as i32, edge4[j] as i32);
                let temp = edge4[0];
                for i in 0..j {
                    edge4[i as usize] = edge4[i as usize + 1];
                    //println!("edge4: {:?}", edge4[i as usize] as i32);
                }
                edge4[j as usize] = temp;

                k = k + 1;
            }
            //println!("Finished {}", j);
            b = (j + 1) * b + k;
        }

        self.ud_sorted_slice = 24 * a + b as i32
         */
    }
    //uuuuuuuuubffbrfdbdlbrlfllfbflrdddfflflbblddrdrrlrbrbdr
    /// Calculates the phase two edge permutation.
    ///
    /// Calculates a description of the edge coordinates, but is only valid
    /// in phase two of the algorithm.
    pub fn calculate_phase_two_edge_permutation(&mut self) {
        let mut x = 0_i32;
        let mut edges: Vec<i32> = Vec::new();

        for i in 0..12 {
            if self.edges[i].coordinate != edge_cubies::Edge::FL
                && self.edges[i].coordinate != edge_cubies::Edge::FR
                && self.edges[i].coordinate != edge_cubies::Edge::BL
                && self.edges[i].coordinate != edge_cubies::Edge::BR
            {
                edges.push(self.edges[i].coordinate as i32);
            }
        }

        for i in (1..8_i32).rev() {
            let mut s = 0_i32;
            for j in (0..(i)).rev() {
                if (edges[j as usize]) > (edges[i as usize]) {
                    s = s + 1;
                }
            }
            x = (x + s) * (edges[i as usize] + 1);
        }
        self.phase_two_edge_permutation = x;
    }

    /// Calculates the parity of the corner permutation.
    /// Used only for testing if the cube can be solved.
    pub fn calculate_corner_parity(&mut self) {
        let mut s = 0;
        for i in (0..8).rev() {
            for j in (0..i).rev() {
                if (self.corners[j].coordinate) > self.corners[i].coordinate {
                    s = s + 1;
                }
            }
        }

        self.corner_parity = s % 2;
    }

    /// Calculates the parity of the edge permutation.
    /// Used only for testing if the cube can be solved.
    pub fn calculate_edge_parity(&mut self) {
        let mut s = 0;
        for i in (0..12).rev() {
            for j in (0..i).rev() {
                if self.edges[j].coordinate as i32 > self.edges[i].coordinate as i32 {
                    s = s + 1;
                }
            }
        }
        self.edge_parity = s % 2
    }

    /// Functions to be called after each move.c
    ///
    /// Used to update the internal state of the variables in the struct
    /// after movements.
    pub fn coordinate_adjustments(&mut self) {
        self.calculate_corner_orientation();
        self.calculate_corner_permutation();
        self.calculate_edge_orientation();
        self.calculate_corner_parity();
        self.calculate_edge_parity();
        self.calculate_ud_slice();
        self.calculate_ud_sorted_slice();
        //println!("Ud sorted done.");
        self.calculate_phase_two_edge_permutation();
        //println!("Phase two edge perm done.");
    }

    /// A clockwise front move.
    pub fn f(&mut self) {
        for i in 0..8 {
            self.corners[i].f();
        }
        for i in 0..12 {
            self.edges[i].f();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise back move.
    pub fn b(&mut self) {
        for i in 0..8 {
            self.corners[i].b();
        }
        for i in 0..12 {
            self.edges[i].b();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise left move.
    pub fn l(&mut self) {
        for i in 0..8 {
            self.corners[i].l();
        }
        for i in 0..12 {
            self.edges[i].l();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise right move.
    pub fn r(&mut self) {
        for i in 0..8 {
            self.corners[i].r();
        }
        for i in 0..12 {
            self.edges[i].r();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise upper move.
    pub fn u(&mut self) {
        for i in 0..8 {
            self.corners[i].u();
        }
        for i in 0..12 {
            self.edges[i].u();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise down move.
    pub fn d(&mut self) {
        for i in 0..8 {
            self.corners[i].d();
        }
        for i in 0..12 {
            self.edges[i].d();
        }
        self.coordinate_adjustments();
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    fn rubiks_dummy(
        corners: [corner_cubies::Corner; 8],
        corners_orientation_values: [i32; 8],
        edges: [edge_cubies::Edge; 12],
        edge_orientation_values: [i32; 12],
    ) -> Cube {
        let mut test_rubiks = Cube::new();
        for i in 0..12 {
            test_rubiks.edges[i].coordinate = edges[i];
            test_rubiks.edges[i].orientation = edge_orientation_values[i]
        }
        for i in 0..8 {
            test_rubiks.corners[i].coordinate = corners[i];
            test_rubiks.corners[i].orientation = corners_orientation_values[i]
        }

        test_rubiks
    }

    fn test_cube_1() -> Cube {
        rubiks_dummy(
            [
                corner_cubies::Corner::DRB,
                corner_cubies::Corner::ULB,
                corner_cubies::Corner::UFL,
                corner_cubies::Corner::DFR,
                corner_cubies::Corner::DBL,
                corner_cubies::Corner::URF,
                corner_cubies::Corner::DLF,
                corner_cubies::Corner::UBR,
            ],
            [1, 1, 2, 2, 0, 0, 0, 0],
            [
                edge_cubies::Edge::UR,
                edge_cubies::Edge::UF,
                edge_cubies::Edge::BR,
                edge_cubies::Edge::FL,
                edge_cubies::Edge::DF,
                edge_cubies::Edge::UL,
                edge_cubies::Edge::DB,
                edge_cubies::Edge::UB,
                edge_cubies::Edge::FR,
                edge_cubies::Edge::BL,
                edge_cubies::Edge::DL,
                edge_cubies::Edge::DR,
            ],
            [0, 2, 1, 1, 2, 0, 0, 0, 2, 1, 2, 2],
        )
    }

    #[test]
    fn test_calculate_corner_orientation() {
        let mut test = test_cube_1();
        test.calculate_corner_orientation();
        assert_eq!(test.corner_orientation, 1188);
    }
    #[test]
    fn test_calculate_corner_permutation() {
        let mut test = test_cube_1();
        test.calculate_corner_permutation();
        assert_eq!(test.corner_permutation, 22235);
    }
    #[test]
    fn test_calculate_edge_orientation() {
        let mut test = test_cube_1();
        test.calculate_edge_orientation();
        assert_eq!(test.edge_orientation, 3098);
    }
    #[test]
    fn test_calculate_ud_slice() {
        let mut test = test_cube_1();
        test.calculate_ud_slice();
        assert_eq!(test.ud_slice, 307);
    }
    #[test]
    // The reason we have a ud_sorted_slice outside of its region is because
    // it is not a G1 state cube.
    fn test_calculate_ud_sorted_slice() {
        let mut test = test_cube_1();
        test.calculate_ud_slice();
        test.calculate_ud_sorted_slice();
        assert_eq!(test.ud_sorted_slice, 7385);
    }

    #[test]
    fn test_calculate_phase_two_edge_permutation() {
        let mut test = test_cube_1();
        test.calculate_phase_two_edge_permutation();
        assert_eq!(test.phase_two_edge_permutation, 131364);
    }

    fn test_coordinate_adjustments() {}

    #[test]
    fn test_f() {
        let mut rubiks = Cube::new();
        let mut test_rubiks = rubiks_dummy(
            [
                corner_cubies::Corner::UFL,
                corner_cubies::Corner::DLF,
                corner_cubies::Corner::ULB,
                corner_cubies::Corner::UBR,
                corner_cubies::Corner::URF,
                corner_cubies::Corner::DFR,
                corner_cubies::Corner::DBL,
                corner_cubies::Corner::DRB,
            ],
            [1, 2, 0, 0, 2, 1, 0, 0],
            [
                edge_cubies::Edge::UR,
                edge_cubies::Edge::FL,
                edge_cubies::Edge::UL,
                edge_cubies::Edge::UB,
                edge_cubies::Edge::DR,
                edge_cubies::Edge::FR,
                edge_cubies::Edge::DL,
                edge_cubies::Edge::DB,
                edge_cubies::Edge::UF,
                edge_cubies::Edge::DF,
                edge_cubies::Edge::BL,
                edge_cubies::Edge::BR,
            ],
            [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1],
        );

        test_rubiks.corner_orientation = 1236;
        test_rubiks.edge_orientation = 1100;
        test_rubiks.ud_slice = 33;

        rubiks.f();
        for i in 0..8 {
            assert_eq!(
                rubiks.corners[i].coordinate,
                test_rubiks.corners[i].coordinate
            );
            assert_eq!(rubiks.edges[i].coordinate, test_rubiks.edges[i].coordinate);
        }
        for i in 8..12 {
            assert_eq!(rubiks.edges[i].coordinate, test_rubiks.edges[i].coordinate);
        }

        assert_eq!(rubiks.corner_orientation, test_rubiks.corner_orientation);
        assert_eq!(rubiks.edge_orientation, test_rubiks.edge_orientation);

        assert_eq!(rubiks.ud_slice, test_rubiks.ud_slice);
    }

    fn test_b() {}

    fn test_l() {}

    fn test_r() {}

    fn test_u() {}

    fn test_d() {}
}
