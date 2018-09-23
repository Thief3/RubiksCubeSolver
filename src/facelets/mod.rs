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

use physical::corner_cubies::*;
use physical::edge_cubies::*;
use physical::Cube;

/// A enum of the different possible face values.
#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct Face {
    facelets_first_half: [Facelets; 27],
    facelets_second_half: [Facelets; 27],
}

impl Face {
    /// Creates a new `Face` from a string of 54 characters. Panics is
    /// string is invalid.
    ///
    /// # Returns
    /// * `Face` - Create a mixed up face with the string provided.
    pub fn new(s: &str) -> Face {
        let mut new_face = Face {
            facelets_first_half: [Facelets::U; 27],
            facelets_second_half: [Facelets::D; 27],
        };
        if s.chars().count() < 54 {
            panic!(
                "Error creating face. Passed string is too short at {}.",
                s.chars().count()
            )
        } else if s.chars().count() > 54 {
            panic!(
                "Error creating face. Passed string is too long at {}.",
                s.chars().count()
            )
        }
        for (i, c) in s.chars().enumerate() {
            if c.to_ascii_lowercase() == 'u' {
                new_face.set_facelets(i, Facelets::U);
            } else if c.to_ascii_lowercase() == 'd' {
                new_face.set_facelets(i, Facelets::D);
            } else if c.to_ascii_lowercase() == 'l' {
                new_face.set_facelets(i, Facelets::L);
            } else if c.to_ascii_lowercase() == 'r' {
                new_face.set_facelets(i, Facelets::R);
            } else if c.to_ascii_lowercase() == 'f' {
                new_face.set_facelets(i, Facelets::F);
            } else if c.to_ascii_lowercase() == 'b' {
                new_face.set_facelets(i, Facelets::B);
            } else {
                panic!("Error creating face. Contains weird characters: {}", c)
            }
        }
        new_face
    }

    /// Creates a new face with default pristine cube values.
    ///
    /// # Returns
    /// * `Face` - A pristine cube face.
    pub fn new_clean() -> Face {
        let mut new_face = Face {
            facelets_first_half: [Facelets::U; 27],
            facelets_second_half: [Facelets::D; 27],
        };
        let facelets_vals = [
            Facelets::U,
            Facelets::R,
            Facelets::F,
            Facelets::D,
            Facelets::L,
            Facelets::B,
        ];
        for i in 0..3 {
            for j in 0..9 {
                new_face.facelets_first_half[i * 9 + j] = facelets_vals[i];
            }
        }
        for i in 3..6 {
            for j in 0..9 {
                new_face.facelets_second_half[i * 9 + j] = facelets_vals[i];
            }
        }
        new_face
    }

    /// A setter method for the facelets arrays in `Cube`. This allows us to
    /// manage the two halfs of the array as one.
    ///
    /// # Parameters
    /// * `index` - The index of which you wish to change. Between 0 and 53
    /// * `val` - The value you wish to change the specific face to.
    pub fn set_facelets(&mut self, index: usize, val: Facelets) {
        if index < 27 && index >= 0 {
            self.facelets_first_half[index] = val;
        } else if index >= 27 && index <= 53 {
            self.facelets_second_half[index - 27] = val;
        } else {
            panic!("set_facelets: Outside the index range for facelets. Keep index within 0 and 53. Index found: {}", index);
        }
    }

    /// A getter method fo rthe facelet arrays in `Cube`. This allows us to
    /// manage the two halfs of the array as one.
    ///
    /// # Parameters
    /// * `index` - The index of the facelets arrays you wish to access, must
    ///    be between 0 and 53 or the function will panic.
    pub fn get_facelets(&self, index: usize) -> Facelets {
        if index < 27 && index >= 0 {
            return self.facelets_first_half[index];
        } else if index >= 27 && index <= 53 {
            return self.facelets_second_half[index - 27];
        } else {
            panic!("get_facelets: Outside the index range for facelets. Keep index within 0 and 53. Index found: {}", index);
        }
    }

    /// A method that checks that the current face is solveable.
    /// # Returns
    /// * `usize` - Returns an error code. Errors can stack and the lower
    ///    number errors take precedance. Can hold multiple values:
    ///
    ///      0 -> `Face` can be solved.
    ///      1 -> Not 9 facelets of each colour
    ///      2 -> Edges aren't the right colours
    ///      3 -> Corners aren't the right colours.
    ///      4 -> Corner and Edge Parity aren't equal.
    ///      5 -> Total Edge Flip is wrong.
    ///      6 -> Total Corner Twist is wrong.
    pub fn check_if_can_be_solved(&self) -> usize {
        let mut return_code = 99;
        let my_cube = self.turn_into_cube();
        if !self.check_all_colours_present() {
            return_code = 1
        } else if !self.check_edges_colours() {
            return_code = 2
        } else if !self.check_corners_colours() {
            return_code = 3
        } else if my_cube.edge_parity != my_cube.corner_parity {
            return_code = 4
        } else if !self.check_edge_flip(my_cube) {
            return_code = 5
        } else if !self.check_corner_twist(my_cube) {
            return_code = 6
        } else {
            return_code = 0
        }

        return_code
    }

    /// A method that checks if all 6 colours have 9 facelets representing them.
    ///
    /// # Returns
    /// * `bool` - Returns true if all 6 colours have 9 facelets representing
    ///   them.
    fn check_all_colours_present(&self) -> bool {
        let mut colour_counts = [0, 0, 0, 0, 0, 0];
        let mut return_bool = false;
        for i in 0..54 {
            if self.get_facelets(i) == Facelets::U {
                colour_counts[0] = colour_counts[0] + 1
            } else if self.get_facelets(i) == Facelets::R {
                colour_counts[1] = colour_counts[1] + 1
            } else if self.get_facelets(i) == Facelets::F {
                colour_counts[2] = colour_counts[2] + 1
            } else if self.get_facelets(i) == Facelets::L {
                colour_counts[3] = colour_counts[3] + 1
            } else if self.get_facelets(i) == Facelets::D {
                colour_counts[4] = colour_counts[4] + 1
            } else if self.get_facelets(i) == Facelets::B {
                colour_counts[5] = colour_counts[5] + 1
            }
        }
        if colour_counts[0] != 9
            || colour_counts[1] != 9
            || colour_counts[2] != 9
            || colour_counts[3] != 9
            || colour_counts[4] != 9
            || colour_counts[5] != 9
        {
            return_bool = false
        } else {
            return_bool = true
        }
        return_bool
    }

    /// A method to test if the corners are all present in some form with the
    /// the correct colours.
    ///
    /// # Returns
    /// * `bool` -> True if all corners exist with the right colours.
    fn check_corners_colours(&self) -> bool {
        let enum_list = [
            Corner::URF,
            Corner::UFL,
            Corner::ULB,
            Corner::UBR,
            Corner::DFR,
            Corner::DLF,
            Corner::DBL,
            Corner::DRB,
        ];
        let mut master_count = 0;
        let mut return_bool = true;
        for i in 0..8 {
            let mut current_colours: Vec<Facelets> = Vec::new();
            for j in 0..3 {
                current_colours.push(self.get_facelets(corner_indexes[i][j] as usize));
            }
            for k in 0..8 {
                let mut count = 0;
                for l in current_colours.iter() {
                    if corner_colours(enum_list[k]).contains(l) {
                        count = count + 1;
                    }
                }
                if count == 3 {
                    master_count = master_count + 1;
                }
            }
        }
        if master_count == 8 {
            return_bool = true
        } else {
            return_bool = false
        }
        return_bool
    }

    /// A method to test that edges are all there with the right colours.
    ///
    /// # Returns
    /// * `bool` - True if all the right colours are indeed there.
    fn check_edges_colours(&self) -> bool {
        let enum_list = [
            Edge::UR,
            Edge::UF,
            Edge::UL,
            Edge::UB,
            Edge::DR,
            Edge::DF,
            Edge::DL,
            Edge::DB,
            Edge::FR,
            Edge::FL,
            Edge::BL,
            Edge::BR,
        ];
        let mut master_count = 0;
        let mut return_bool = true;
        for i in 0..8 {
            let mut current_colours: Vec<Facelets> = Vec::new();
            for j in 0..2 {
                current_colours.push(self.get_facelets(edge_indexes[i][j] as usize));
            }
            for k in 0..12 {
                let mut count = 0;
                for l in current_colours.iter() {
                    if edge_colours(enum_list[k]).contains(l) {
                        count = count + 1;
                    }
                }
                if count == 2 {
                    master_count = master_count + 1;
                }
            }
        }
        if master_count == 8 {
            return_bool = true
        } else {
            return_bool = false
        }
        return_bool
    }

    /// Checks the edge flip of `c`.
    ///
    /// # Parameters
    /// * `c` - A `Cube` to check the flip of.
    /// # Returns
    /// * `bool` - Returns true if the cube has a solveable flip.
    fn check_edge_flip(&self, c: Cube) -> bool {
        let mut s = 0;
        let mut return_bool = true;
        for e in c.edges.iter() {
            s = s + e.orientation;
        }
        if s % 2 != 0 {
            return_bool = false;
        }
        return_bool
    }

    /// Checks the corners twist of `c`.
    ///
    /// # Parameters
    /// * `c` - A `Cube` to check the twist of.
    /// # Returns
    /// * `bool` - Returns true if the cube has a solveable twist.
    fn check_corner_twist(&self, c: Cube) -> bool {
        let mut s = 0;
        let mut return_bool = true;
        for cor in c.corners.iter() {
            s = s + cor.orientation;
        }
        if s % 3 != 0 {
            return_bool = false;
        }
        return_bool
    }

    /// A method to turn a face into a cube.
    /// A heavy amount of the code was ported from (https://github.com/hkociemba/RubiksCube-TwophaseSolver/blob/master/face.py).
    /// # Returns
    /// * `Cube` - A `Cube` with values homomorphic to this face.
    pub fn turn_into_cube(&self) -> Cube {
        let mut new_cube = Cube::new();

        let edges = [
            Edge::UR,
            Edge::UF,
            Edge::UL,
            Edge::UB,
            Edge::DR,
            Edge::DF,
            Edge::DL,
            Edge::DB,
            Edge::FR,
            Edge::FL,
            Edge::BL,
            Edge::BR,
        ];

        let corners = [
            Corner::URF,
            Corner::UFL,
            Corner::ULB,
            Corner::UBR,
            Corner::DFR,
            Corner::DLF,
            Corner::DBL,
            Corner::DRB,
        ];

        // Basically this entire algorithm was recreated from
        // https://github.com/hkociemba/RubiksCube-TwophaseSolver/blob/master/face.py
        for (i, dud) in corners.iter().enumerate() {
            let fac = corner_indexes[i];
            let mut col1: Facelets;
            let mut col2: Facelets;
            let mut o: usize = 0;
            for ori in 0..3 {
                if self.get_facelets(fac[ori]) == Facelets::U
                    || self.get_facelets(fac[ori]) == Facelets::D
                {
                    o = ori;
                    break;
                }
            }
            col1 = self.get_facelets(fac[(o + 1) % 3]);
            col2 = self.get_facelets(fac[(o + 2) % 3]);

            for c in corners.iter() {
                let col = corner_colours(*c);
                if col1 == col[1] && col2 == col[2] {
                    new_cube.corners[i] = CornerCubie::new(*c);
                    new_cube.corners[i].orientation = o as i32;
                    break;
                }
            }

            for (i, dud) in edges.iter().enumerate() {
                for e in edges.iter() {
                    if self.get_facelets(edge_indexes[i][0]) == edge_colours(*e)[0]
                        && self.get_facelets(edge_indexes[i][1]) == edge_colours(*e)[1]
                    {
                        new_cube.edges[i] = EdgeCubie::new(*e);
                        new_cube.edges[i].orientation = 0;
                    } else if self.get_facelets(edge_indexes[i][0]) == edge_colours(*e)[1]
                        && self.get_facelets(edge_indexes[i][1]) == edge_colours(*e)[0]
                    {
                        new_cube.edges[i] = EdgeCubie::new(*e);
                        new_cube.edges[i].orientation = 1;
                    }
                }
            }
        }

        new_cube.coordinate_adjustments();
        new_cube
    }
}

/// ****************************************************************************
/// * Definitions
/// ****************************************************************************
/// A list of all the edges and their index in face. Already in order.
const edge_indexes: [[usize; 2]; 12] = [
    [5, 10],
    [7, 19],
    [3, 37],
    [1, 46],
    [32, 16],
    [28, 25],
    [30, 43],
    [34, 52],
    [23, 12],
    [21, 41],
    [50, 39],
    [48, 14],
];
/// A pattern matching method that takes an `Edges` and returns the two
/// `Facelets` that belong to that edge.
///
/// # Parameters
/// * `e` - An edge to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 2]` - The two `Facelets` connected to `e`.
pub fn edge_colours(e: Edge) -> [Facelets; 2] {
    match e {
        Edge::UR => [Facelets::U, Facelets::R],
        Edge::UF => [Facelets::U, Facelets::F],
        Edge::UL => [Facelets::U, Facelets::L],
        Edge::UB => [Facelets::U, Facelets::B],
        Edge::DR => [Facelets::D, Facelets::R],
        Edge::DF => [Facelets::D, Facelets::F],
        Edge::DL => [Facelets::D, Facelets::L],
        Edge::DB => [Facelets::D, Facelets::B],
        Edge::FR => [Facelets::F, Facelets::R],
        Edge::FL => [Facelets::F, Facelets::L],
        Edge::BL => [Facelets::B, Facelets::L],
        Edge::BR => [Facelets::B, Facelets::R],
    }
}

/// A list of all the corners and their indexes in `Face`. Already in order.
const corner_indexes: [[usize; 3]; 8] = [
    [8, 9, 20],
    [6, 18, 38],
    [0, 36, 47],
    [2, 45, 11],
    [29, 26, 15],
    [27, 44, 24],
    [33, 53, 42],
    [35, 17, 51],
];

/// A pattern matching method that takes a `Corners` and returns the three
/// `Facelets` that belong to that corner.
///
/// # Parameters
/// * `c` - An corner to get the corresponding `Facelets`.
/// # Returns
/// * `[Facelets; 3]` - The two `Facelets` connected to `c`.
pub fn corner_colours(c: Corner) -> [Facelets; 3] {
    match c {
        Corner::URF => [Facelets::U, Facelets::R, Facelets::F],
        Corner::UFL => [Facelets::U, Facelets::F, Facelets::L],
        Corner::ULB => [Facelets::U, Facelets::L, Facelets::B],
        Corner::UBR => [Facelets::U, Facelets::B, Facelets::R],
        Corner::DFR => [Facelets::D, Facelets::F, Facelets::R],
        Corner::DLF => [Facelets::D, Facelets::L, Facelets::F],
        Corner::DBL => [Facelets::D, Facelets::B, Facelets::L],
        Corner::DRB => [Facelets::D, Facelets::R, Facelets::B],
    }
}
