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

pub fn phase_one_search(
    rubiks: physical::Cube,
    depth: usize,
    move_list: Vec<solver::Moves>,
) -> Vec<solver::Moves> {
    let current_moves = move_list.clone();
    let c = rubiks.clone();
    let mut result;
    let mut current_depth = depth;
    while !phase_one_subgoal(c) {
        result = phase_one_tree_search(
            c,
            solver::MAX_PHASE_ONE_DEPTH - current_depth,
            move_list.clone(),
        );
        if result.0 == true {
            break;
        } else if current_depth == 0 {
            println!("No solution was found, so something is very broken");
            break;
        } else {
            current_depth = current_depth - 1;
        }
    }
    current_moves
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn phase_one_tree_search(
    rubiks: physical::Cube,
    depth: usize,
    move_list: Vec<solver::Moves>,
) -> (bool, Vec<solver::Moves>) {
    let mut current_moves = move_list.clone();
    let mut current_depth = depth;
    let mut c = rubiks.clone();
    let mut result =(false, Vec::new());
    while current_depth > 0 {
        for movement in PHASE_ONE_MOVE_LIST.iter() {
            let unwrapped = move_list.last().unwrap();

            if move_list.len() == 0 ||
                (*unwrapped != *movement &&
                *unwrapped == solver::Moves::NONE ||
                (*unwrapped != solver::opposite_move(*unwrapped) &&
                *unwrapped != solver::cannot_follow(*unwrapped))) {

                solver::do_move(&mut c, *movement);
                current_moves.push(*movement);

                println!("Corner Orientation: {:?}\nEdge Orientation: {:?}\nUD Slice: {:?}", rubiks.corner_orientation, rubiks.edge_orientation, rubiks.ud_slice,);
                
                if phase_one_subgoal(c) {
                    result = (true, current_moves.clone())
                } else {
                    result = (false, Vec::new())
                }
            }
        }
        println!("Current Depth: {:?}", current_depth);
        current_depth = current_depth - 1;
    }
    println!("Depth");
    result 
}

fn phase_one_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0) && (rubiks.ud_slice == 0)
}
