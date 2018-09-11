//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

pub mod phase_one;
pub mod phase_two;
use physical;
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Moves {
    F1,
    F2,
    F3,
    B1,
    B2,
    B3,
    U1,
    U2,
    U3,
    D1,
    D2,
    D3,
    L1,
    L2,
    L3,
    R1,
    R2,
    R3,
    NONE,
}

pub static MAX_PHASE_ONE_DEPTH: usize = 12;

pub fn search(rubiks: &mut physical::Cube) {
    let mut a = Vec::new();
    a.push(Moves::NONE);
    let g1_state_move_list = phase_one::phase_one_search(*rubiks, 12, a);
    println!("Move list: {:?}", g1_state_move_list);
    //let pristine_state_move_list = phase_two::phase_two_search(rubiks, 5, &a);
    //println!("Move list two: {:?}", pristine_state_move_list);
}

pub fn do_move(rubiks: &mut physical::Cube, movement: Moves) {
    match movement {
        Moves::F1 => rubiks.f(),
        Moves::F2 => {
            rubiks.f();
            rubiks.f()
        }
        Moves::F3 => {
            rubiks.f();
            rubiks.f();
            rubiks.f()
        }
        Moves::B1 => rubiks.b(),
        Moves::B2 => {
            rubiks.b();
            rubiks.b()
        }
        Moves::B3 => {
            rubiks.b();
            rubiks.b();
            rubiks.b()
        }
        Moves::U1 => rubiks.u(),
        Moves::U2 => {
            rubiks.u();
            rubiks.u()
        }
        Moves::U3 => {
            rubiks.u();
            rubiks.u();
            rubiks.u()
        }
        Moves::D1 => rubiks.d(),
        Moves::D2 => {
            rubiks.d();
            rubiks.d()
        }
        Moves::D3 => {
            rubiks.d();
            rubiks.d();
            rubiks.d()
        }
        Moves::L1 => rubiks.l(),
        Moves::L2 => {
            rubiks.l();
            rubiks.l()
        }
        Moves::L3 => {
            rubiks.l();
            rubiks.l();
            rubiks.l()
        }
        Moves::R1 => rubiks.r(),
        Moves::R2 => {
            rubiks.r();
            rubiks.r()
        }
        Moves::R3 => {
            rubiks.r();
            rubiks.r();
            rubiks.r()
        }
        Moves::NONE => {}
    }
}

pub fn opposite_move(movement: Moves) -> Moves {
    let a = match movement {
        Moves::F1 => Moves::B1,
        Moves::F2 => Moves::B2,
        Moves::F3 => Moves::B3,
        Moves::B1 => Moves::F1,
        Moves::B2 => Moves::F2,
        Moves::B3 => Moves::F3,
        Moves::U1 => Moves::D1,
        Moves::U2 => Moves::D2,
        Moves::U3 => Moves::D3,
        Moves::D1 => Moves::U1,
        Moves::D2 => Moves::U2,
        Moves::D3 => Moves::U3,
        Moves::L1 => Moves::R1,
        Moves::L2 => Moves::R2,
        Moves::L3 => Moves::R3,
        Moves::R1 => Moves::L1,
        Moves::R2 => Moves::L2,
        Moves::R3 => Moves::L3,
        Moves::NONE => Moves::NONE,
    };
    a
}

pub fn cannot_follow(movement: Moves) -> Moves {
    let a = match movement {
        Moves::F1 => Moves::F2,
        Moves::F2 => Moves::F1,
        Moves::F3 => Moves::F2,
        Moves::B1 => Moves::B2,
        Moves::B2 => Moves::B1,
        Moves::B3 => Moves::B2,
        Moves::U1 => Moves::U2,
        Moves::U2 => Moves::U1,
        Moves::U3 => Moves::U2,
        Moves::D1 => Moves::D2,
        Moves::D2 => Moves::D1,
        Moves::D3 => Moves::D2,
        Moves::L1 => Moves::L2,
        Moves::L2 => Moves::L1,
        Moves::L3 => Moves::L2,
        Moves::R1 => Moves::R2,
        Moves::R2 => Moves::R1,
        Moves::R3 => Moves::R2,
        Moves::NONE => Moves::NONE,
    };
    a
}
