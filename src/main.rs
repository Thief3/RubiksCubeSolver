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
    /*
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::F1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::F1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::F1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::F1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::B1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::B1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::B1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    c = solver::do_move(c, solver::Moves::B1);
    println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
     */
    c = solver::do_move(c, solver::Moves::F1);
    println!(
        "C_O: {}, E_O: {}, UD: {}",
        c.corner_orientation, c.edge_orientation, c.ud_slice
    );
    c = solver::do_move(c, solver::Moves::U1);
    println!(
        "C_O: {}, E_O: {}, UD: {}",
        c.corner_orientation, c.edge_orientation, c.ud_slice
    );
    //c = solver::do_move(c, solver::Moves::U3);
    //println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
    //c = solver::do_move(c, solver::Moves::F3);
    //println!("C_O: {}, E_O: {}, UD: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);

    solver::search(&mut c);
}
