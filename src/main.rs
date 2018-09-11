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
    /*c.d();
    c.d();
    c.u();
    c.u();
    c.u();
    c.r();
    c.r();
    c.u();
    c.f();
    c.f();
    c.d();
    c.d();
    c.u();
    c.u();
    c.u();
    c.r();
    c.r();
    c.u();
    c.u();
    c.u();
    c.b();
    c.b();
    c.b();
    c.l();
    c.l();
    c.b();
    c.b();
    c.b();
    c.d();
    c.d();
    c.u();
    c.b();
    c.b();
    c.l();
    c.l();
    c.l();
    c.d();
    c.d();
    c.d();
    c.r();
    c.r();*/
    c.r();
    c.r();
    c.u();
    c.r();
    c.f();
    c.d();
    solver::search(&mut c);
    //println!("{}", utility::factorial(0));
}
