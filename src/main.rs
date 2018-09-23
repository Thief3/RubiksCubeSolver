//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

mod facelets;
mod physical;
mod solver;
mod utility;

fn main() {
    let mut test = facelets::Face::new("UUUUUULLLURRURRURRFFFFFFFFFRRRDDDDDDLLDLLDLLDBBBBBBBBB");
    let mut my_cube = test.turn_into_cube();
    let mut origin_cube = physical::Cube::new();
    let mut real_cube = physical::Cube::new();
    real_cube.f();
    println!("*************EDGES************");
    for i in 0..12 {
        println!("Coordinate  Real: {:?}  Test: {:?}  Original: {:?}", real_cube.edges[i].coordinate, my_cube.edges[i].coordinate, origin_cube.edges[i].coordinate);
        println!("Orientation Real: {}   Test: {}   Original: {}", real_cube.edges[i].orientation, my_cube.edges[i].orientation, origin_cube.edges[i].orientation);
    }
    println!("@@@@@@@@@@@@@CORNERS@@@@@@@@@@@@@@");
    for i in 0..8 {
        println!("Coordinate  Real: {:?}  Test: {:?}  Original: {:?}", real_cube.corners[i].coordinate, my_cube.corners[i].coordinate, origin_cube.corners[i].coordinate);
        println!("Orientation Real: {}    Test: {}  Original: {}", real_cube.corners[i].orientation, my_cube.corners[i].orientation, origin_cube.corners[i].orientation);
    }
   
    //solver::complete_search(&mut real_cube);
    println!("Return Code: {}", test.check_if_can_be_solved());
   
}
