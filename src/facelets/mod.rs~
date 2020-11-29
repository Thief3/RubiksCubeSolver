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


pub type RubiksChar = [char; 54];
pub type RubiksFacelets = [Facelets; 54];
pub type Face = RubiksFacelets;

pub trait IFace {
    fn new(&str) -> Face;
    fn new_clean() -> Face;
    fn set_facelets(&mut self, index: usize, val: Facelets);
    fn get_facelets(&self, index: usize) -> Facelets;
    fn check_if_can_be_solved(&self) -> usize;
    fn check_all_colours_present(&self) -> bool;
    fn check_corners_colours(&self) -> bool;
    fn check_edges_colours(&self) -> bool;
    fn check_edge_flip(&self, c: Cube) -> bool;
    fn check_corner_twist(&self, c: Cube) -> bool;
    fn turn_into_cube(&self) -> Cube;
    fn return_code_matcher(&self) -> (&'static str, bool);
}

impl IFace for Face {
    /// Creates a new `Face` from a string of 54 characters. Panics is
    /// string is invalid.
    ///
    /// # Returns
    /// * `Face` - Create a mixed up face with the string provided.
    fn new(s: &str) -> Face {
        let mut new_face: Face = [Facelets::U; 54];

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
        //println!("Face Created!");
        new_face
    }

    #[allow(dead_code)]
    /// Creates a new face with default pristine cube values.
    ///
    /// # Returns
    /// * `Face` - A pristine cube face.
    fn new_clean() -> Face {
        let mut new_face: Face = [Facelets::U; 54];
        
        let facelets_vals = [
            Facelets::U,
            Facelets::R,
            Facelets::F,
            Facelets::L,
            Facelets::B,
            Facelets::D,
        ];
        for i in 0..6 {
            for j in 0..9 {
                new_face[i * 9 + j] = facelets_vals[i];
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
    fn set_facelets(&mut self, index: usize, val: Facelets) {
        if index < 54{
            self[index] = val;
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
    fn get_facelets(&self, index: usize) -> Facelets {
        if index < 54 {
            return self[index]
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
    fn check_if_can_be_solved(&self) -> usize {
        let return_code;
        let my_cube = self.turn_into_cube();
        //println!("Can be solved? {:?}", my_cube);
        if !self.check_all_colours_present() {
            return_code = 1;
        }
        else if !self.check_edges_colours() {
            return_code = 2;
        }
        else if !self.check_corners_colours() {
            return_code = 3;
        }
        else if my_cube.edge_parity() != my_cube.corner_parity() {
            return_code = 4;
        }
        else if !self.check_edge_flip(my_cube) {
            return_code = 5;
        }
        else if !self.check_corner_twist(my_cube) {
            return_code = 6;
        }
        else {
            return_code = 0;
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
        let return_bool;
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
        let mut master_count = 0;
        let return_bool;
        for i in 0..8 {
            let mut current_colours: Vec<Facelets> = Vec::new();
            for j in 0..3 {
                current_colours.push(self.get_facelets(CORNER_INDEXES[i][j] as usize));
            }
            for k in 0..8 {
                let mut count = 0;
                for l in current_colours.iter() {
                    if corner_colours(CORNER_LIST[k]).contains(l) {
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
        let mut master_count = 0;
        let return_bool: bool;
        let mut cur_edges: [[Facelets; 2]; 12] = [[Facelets::U, Facelets::U]; 12];

        for edge in 0..EDGE_INDEXES.len() {
            cur_edges[edge] = [self.get_facelets(EDGE_INDEXES[edge][0] as usize), self.get_facelets(EDGE_INDEXES[edge][1] as usize)];
        }
        
        print!("Edges: {:?}\n", cur_edges);

        for i in 0..EDGE_INDEXES.len(){
            if !cur_edges.iter()
                .any(|v|
                     (v[0] == edge_colours(EDGE_LIST[i])[0] && v[1] == edge_colours(EDGE_LIST[i])[1]) ||
                     (v[0] == edge_colours(EDGE_LIST[i])[1] && v[1] == edge_colours(EDGE_LIST[i])[0])){
                    print!("Missing {:?}\n", EDGE_LIST[i]);
                    return false;
                }
        }
        
        true
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
    fn turn_into_cube(&self) -> Cube {
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
        print!("Start Cubing.");
        for (i, _dud) in corners.iter().enumerate() {
            let fac = CORNER_INDEXES[i];
            let col1: Facelets;
            let col2: Facelets;
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
        }
        
        for (i, _dud) in edges.iter().enumerate() {
            for e in edges.iter() {
                if self.get_facelets(EDGE_INDEXES[i][0]) == edge_colours(*e)[0]
                    && self.get_facelets(EDGE_INDEXES[i][1]) == edge_colours(*e)[1]
                {
                    new_cube.edges[i] = EdgeCubie::new(*e);
                    new_cube.edges[i].orientation = 0;
                }

                if self.get_facelets(EDGE_INDEXES[i][0]) == edge_colours(*e)[1]
                    && self.get_facelets(EDGE_INDEXES[i][1]) == edge_colours(*e)[0]
                {
                    new_cube.edges[i] = EdgeCubie::new(*e);
                    new_cube.edges[i].orientation = 1;
                }
            }
        }
        // This is the problem
        //println!("New cube coordinates adjusted.");
        println!("New cube who dis?: {:?}\n", new_cube);
        new_cube.r();
        println!("Cube after R: {:?}\n", new_cube);
        println!("Cube Corner Orientation: {:?}\n", new_cube.corner_orientation());
        new_cube
    }
    
    fn return_code_matcher(&self) -> (&'static str, bool) {
        let return_code = self.check_if_can_be_solved();
        println!("Return code is: {}", return_code);
        match return_code {
            0 => {
                return ("Attempting solve...", true);
            },
            1 => return ("You don't have 9 facelets of each colour.", false),
            2 => return ("Not all the edges exist (there may be multiple edges with the same two colours.)", false),
            3 => return ("Not all the corners exist (there may be multiple corners with the same three colours.)", false),
            4 => return ("Edge and Corner parities aren't equal.", false),
            5 => return ("The total Edge flip is wrong.", false),
            6 => return ("The total Corner twist is wrong.", false),
            _ => panic!("How on earth did you get a different return code????"),
        }
    }
}



/// ****************************************************************************
/// * Definitions
/// ****************************************************************************
/// A list of all the edges and their index in face. Already in order.
const EDGE_INDEXES: [[usize; 2]; 12] = [
        [ U6, R2 ], [ U8, F2 ], [ U4, L2 ], [ U2, B2 ], [ D6, R8 ], [ D2, F8 ],
        [ D4, L8 ], [ D8, B8 ], [ F6, R4 ], [ F4, L6 ], [ B6, L4 ], [ B4, R6 ],
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
const CORNER_INDEXES: [[usize; 3]; 8] = [
    [ U9, R1, F3 ], [ U7, F1, L3 ], [ U1, L1, B3 ], [ U3, B1, R3 ],
    [ D3, F9, R7 ], [ D1, L9, F7 ], [ D7, B9, L7 ], [ D9, R9, B7 ],
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

const CORNER_LIST: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::UBR,
    Corner::DFR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
];

const EDGE_LIST: [Edge; 12] = [
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

pub const U1: usize = 0;
pub const U2: usize = 1;
pub const U3: usize = 2;
pub const U4: usize = 3;
pub const U5: usize = 4;
pub const U6: usize = 5;
pub const U7: usize = 6;
pub const U8: usize = 7;
pub const U9: usize = 8;
pub const R1: usize = 9;
pub const R2: usize = 10;
pub const R3: usize = 11;
pub const R4: usize = 12;
pub const R5: usize = 13;
pub const R6: usize = 14;
pub const R7: usize = 15;
pub const R8: usize = 16;
pub const R9: usize = 17;
pub const F1: usize = 18;
pub const F2: usize = 19;
pub const F3: usize = 20;
pub const F4: usize = 21;
pub const F5: usize = 22;
pub const F6: usize = 23;
pub const F7: usize = 24;
pub const F8: usize = 25;
pub const F9: usize = 26;
pub const D1: usize = 27;
pub const D2: usize = 28;
pub const D3: usize = 29;
pub const D4: usize = 30;
pub const D5: usize = 31;
pub const D6: usize = 32;
pub const D7: usize = 33;
pub const D8: usize = 34;
pub const D9: usize = 35;
pub const L1: usize = 36;
pub const L2: usize = 37;
pub const L3: usize = 38;
pub const L4: usize = 39;
pub const L5: usize = 40;
pub const L6: usize = 41;
pub const L7: usize = 42;
pub const L8: usize = 43;
pub const L9: usize = 44;
pub const B1: usize = 45;
pub const B2: usize = 46;
pub const B3: usize = 47;
pub const B4: usize = 48;
pub const B5: usize = 49;
pub const B6: usize = 50;
pub const B7: usize = 51;
pub const B8: usize = 52;
pub const B9: usize = 53;
