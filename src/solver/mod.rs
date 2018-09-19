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

const MAX_PHASE_ONE_DEPTH: usize = 18;

pub fn complete_search(rubiks: &mut physical::Cube) {
    let a = Vec::new();
    let b = Vec::new();
    //    a.push(Moves::F1);
    let mut c = rubiks.clone();
    println!("{:?}", c);
    let g1_state_move_list = search(&mut c, a, MAX_PHASE_ONE_DEPTH, phase_one_subgoal, &PHASE_ONE_MOVE_LIST);
    println!("Move list: {:?}", g1_state_move_list);
    println!("{:?}", c);
    let pristine_state_move_list = search(&mut c, b, MAX_PHASE_TWO_DEPTH, phase_two_subgoal, &PHASE_TWO_MOVE_LIST);
    println!("Move list two: {:?}", pristine_state_move_list);
}

const MAX_PHASE_TWO_DEPTH: usize = 12;
const PHASE_TWO_MOVE_LIST: [Moves; 6] = [
    Moves::U1,
    Moves::B2,
    Moves::F2,
    Moves::D1,
    Moves::L2,
    Moves::R2,
];

fn phase_two_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_permutation == 0)
        && (rubiks.phase_two_edge_permutation == 0)
        && (rubiks.ud_sorted_slice == 0)
}

pub fn do_move(mut rubiks: physical::Cube, movement: Moves) -> physical::Cube {
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
        Moves::NONE => {
            // pass
        }
    }
    rubiks
}

const PHASE_ONE_MOVE_LIST: [Moves; 18] = [
    Moves::F1,
    Moves::F2,
    Moves::F3,
    Moves::B1,
    Moves::B2,
    Moves::B3,
    Moves::U1,
    Moves::U2,
    Moves::U3,
    Moves::D1,
    Moves::D2,
    Moves::D3,
    Moves::L1,
    Moves::L2,
Moves::L3,
    Moves::R1,
    Moves::R2,
    Moves::R3,
];

pub fn search(
    rubiks: &mut physical::Cube,
    move_list: Vec<Moves>,
    max_depth: usize,
    subgoal: fn(physical::Cube) -> bool,
    whole_move_list: &[Moves],
) -> Vec<Moves> {
    let mut results:bool= false;
    let mut c:physical::Cube = rubiks.clone();
    let mut solution: Vec<Moves> = Vec::new();
    if subgoal(rubiks.clone()) {
        results = true;
    }
    for i in 0..max_depth {
        println!("Depth: {}", i);
        tree_search(rubiks, &mut c, i, move_list.clone(), &mut results, &mut solution, whole_move_list, subgoal);
        if results == true {
            //println!("{:?}", c.clone());
            break;
        };
    
    }
    solution
}

//#[cfg_attr(rustfmt, rustfmt_skip)]
fn tree_search(
    rubiks: &mut physical::Cube,
    dummy_rubiks: &mut physical::Cube,
    depth: usize,
    move_list: Vec<Moves>,
    found: &mut bool,
    final_list: &mut Vec<Moves>,
    whole_move_list: &[Moves],
    subgoal: fn(physical::Cube) -> bool,
) {
    if depth > 0 && *found == false {
        for movement in whole_move_list.iter() {
            if *found == false {
                let mut last_move: Moves;
                if move_list.len() != 0 {
                    last_move = *move_list.last().unwrap();
                } else {
                    last_move = Moves::NONE;
                }

                if *movement != last_move
                    && *movement != opposite_move(last_move)
                    && *movement != cannot_follow(last_move)
                {
                    let mut current_list = move_list.clone();
                    let mut c = dummy_rubiks.clone();
                    c = do_move(c, *movement);
                    //                    println!("C_Permutation: {}, E_2_Permutation: {}, UD_SORTED_SLICE: {}", c.corner_permutation, c.phase_two_edge_permutation, c.ud_sorted_slice);
                    current_list.push(*movement);

                    if subgoal(c){
                        *found = true;
                        *final_list = current_list.clone();
//                        println!("This one before: {:?}", rubiks.clone());
                        *rubiks = c.clone();
//                        println!("This one. {:?}", rubiks.clone());
                        break;
                    } else {
                        tree_search(&mut *rubiks, &mut c, depth - 1, current_list.clone(), &mut *found, &mut *final_list, whole_move_list, subgoal);
                    }
                }
            }
        }
    }
}

fn phase_one_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0) && (rubiks.ud_slice == 0)
}

fn opposite_move(movement: Moves) -> Moves {
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

fn cannot_follow(movement: Moves) -> Moves {
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
