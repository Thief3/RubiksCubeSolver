//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

//mod notation;

mod physical;
mod solver;
mod utility;

fn main() {
    let mut c = physical::Cube::new();
    c.calculate_ud_sorted_slice();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    println!("{}", c.ud_sorted_slice);
    c.f();
    //println!("*******************\nEdges: {:?}, \nCorner: {:?}", c.edges, c.corners);
   // solver::complete_search(&mut c);
    println!("{}", c.ud_sorted_slice);
}
