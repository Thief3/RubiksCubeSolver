//! A module for the physical representation of a Rubiks cube.
//!
//! Deals with all the moves a cube has, as well as insuring the
//! transformation are in a group/coordinate style that best works with
//! the two-phase algorithm.

use utility;
mod corner_cubies;

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
/// * `edge_permutation` - A value between 0 and 479001599, representing
///     the permutation of the cubes edges.
/// * `ud_slice` - A value between 0 and 494, representing the front UD
///     slice edges.
/// * `corners` - An array of the 8 `CornerCubie`. 
#[derive(PartialEq, Debug)]
pub struct Cube {
    pub corner_orientation: i32,
    pub edge_orientation: i32,
    pub corner_permutation: i32,
    pub edge_permutation: i32,
    pub ud_slice: i32,
    pub corners: [corner_cubies::CornerCubie; 8],
}

impl Cube {
    /// Creates a new `Cube` object with all values set at start positions.
    pub fn new() -> Cube {
        Cube {
            corner_orientation: 0,
            edge_orientation: 0,
            corner_permutation: 0,
            edge_permutation: 0,
            ud_slice: 0,
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
        }
    }

    /// Calculates the corner orientation.
    /// 
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner orientation of the whole cube.  Further
    /// explanation at (http://kociemba.org/cube.htm)
    pub fn calculate_corner_orientation(&mut self) {
        let mut sum = 0;
        for i in 0..6 {
            sum = sum + self.corners[i].orientation * 3_i32.pow((6 - i) as u32)
        }
        self.corner_orientation = sum
    }

    /// Calculates the corner permutation.
    /// 
    /// Should be called after every movement. Calculates a tenary value used
    /// to represent the corner permutation of the whole cube. Further
    /// explanation at (http://kociemba.org/cube.htm)
    pub fn calculate_corner_permutation(&mut self){
        let mut sum = 0;
        for i in 1..8 {
            let mut diff =  self.corners[i].old_coordinate as i32 - self.corners[i].coordinate as i32;
            if diff == 0 {diff = diff + 1};
            if diff >= 0 {
                
                sum = sum + diff*(utility::factorial(i as i64) as i32);
            }
        }
        self.corner_permutation = sum;
    }

    /// Functions to be called after each move.c
    /// 
    /// Used to update the internal state of the variables in the struct
    /// after movements.
    pub fn coordinate_adjustments(&mut self){
        self.calculate_corner_orientation();
        self.calculate_corner_permutation();
    }

    /// A clockwise front move.
    pub fn f(&mut self) {
        for i in 0..8 {
            self.corners[i].f();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise back move.
    pub fn b(&mut self) {
        for i in 0..8 {
            self.corners[i].b();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise left move.
    pub fn l(&mut self) {
        for i in 0..8 {
            self.corners[i].l();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise right move.
    pub fn r(&mut self) {
        for i in 0..8 {
            self.corners[i].r();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise upper move.
    pub fn u(&mut self) {
        for i in 0..8 {
            self.corners[i].u();
        }
        self.coordinate_adjustments();
    }

    /// A clockwise down move.
    pub fn d(&mut self) {
        for i in 0..8 {
            self.corners[i].d();
        }
        self.coordinate_adjustments();
    }
}
