//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

use physical;
use solver;

static PHASE_ONE_MOVE_LIST: [solver::Moves; 18] = [
    solver::Moves::F1,
    solver::Moves::F2,
    solver::Moves::F3,
    solver::Moves::B1,
    solver::Moves::B2,
    solver::Moves::B3,
    solver::Moves::U1,
    solver::Moves::U2,
    solver::Moves::U3,
    solver::Moves::D1,
    solver::Moves::D2,
    solver::Moves::D3,
    solver::Moves::L1,
    solver::Moves::L2,
    solver::Moves::L3,
    solver::Moves::R1,
    solver::Moves::R2,
    solver::Moves::R3,
];

static mut FOUND: bool = false;

pub fn phase_one_search(
    rubiks: physical::Cube,
    move_list: Vec<solver::Moves>,
) -> Vec<solver::Moves> {
    let mut results = false;
    let mut solution: Vec<solver::Moves> = Vec::new();
    for i in 0..solver::MAX_PHASE_ONE_DEPTH {
        println!("OuterDepth: {}", i);
        results = phase_one_tree_search(rubiks, i, move_list.clone(), &mut results);
        
        println!("Results: {:?}", results);
        if results == true {
            println!("OMG IT WORKED.");
            //solution = results.1;
            break;
        };
    
    }
    println!("Defo got to the end :(");
    solution
}

//#[cfg_attr(rustfmt, rustfmt_skip)]
fn phase_one_tree_search(
    rubiks: physical::Cube,
    depth: usize,
    move_list: Vec<solver::Moves>,
    found: &mut bool,
) -> bool {
    let mut final_list: Vec<solver::Moves> = Vec::new();
    if depth > 0 && *found == false {
        for movement in PHASE_ONE_MOVE_LIST.iter() {
            if *found == false {
                let mut last_move: solver::Moves;
                if move_list.len() != 0 {
                    last_move = *move_list.last().unwrap();
                } else {
                    last_move = solver::Moves::NONE;
                }

                if *movement != last_move
                    && *movement != solver::opposite_move(last_move)
                    && *movement != solver::cannot_follow(last_move)
                {
                    let mut current_list = move_list.clone();
                    let mut c = rubiks.clone();
                    c = solver::do_move(c, *movement);
                    current_list.push(*movement);

                    if phase_one_subgoal(c) {
                        *found = true;
                        println!("I exist");
                        println!("Current List: {:?}", current_list);
                        final_list = current_list.clone();
                        break;
                    } else {
                        phase_one_tree_search(c, depth - 1, current_list.clone(), &mut *found);
                    }
                }
            }
        }
    }
    
        println!("InnerFound: {}", found.clone());
        true
}

fn phase_one_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0) && (rubiks.ud_slice == 0)
}
