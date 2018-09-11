//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//! 
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later. 
//! Some rights reserved. See COPYING, AUTHORS.
//! 
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

use solver;
use physical;

static PHASE_TWO_MOVE_LIST: [solver::Moves; 6] = [
    solver::Moves::U1,
    solver::Moves::B2,
    solver::Moves::F2,
    solver::Moves::D1,
    solver::Moves::L2,
    solver::Moves::R2,
];

pub fn phase_two_search(
    rubiks: &mut physical::Cube,
    depth: i32,
    move_list: &Vec<solver::Moves>,
) -> Vec<solver::Moves> {
    let mut final_moves = move_list.clone();
    let mut acc_final_moves = move_list.clone();
    //println!("Final Moves: {:?}", final_moves);
    let mut my_depth = depth;
    //println!("Depth: {}", my_depth);
    while ((!phase_two_subgoal(rubiks) && my_depth > 0 )) {
        for movement in PHASE_TWO_MOVE_LIST.iter() {
            let unwrapped = final_moves.last().unwrap();
            if final_moves.len() == 0
                || (*unwrapped != *movement &&
                    *unwrapped == solver::Moves::NONE || (
                    *unwrapped != solver::opposite_move(*unwrapped) &&
                    *unwrapped != solver::cannot_follow(*unwrapped))) {
                let mut current_moves = move_list.clone();
                let mut c = rubiks.clone();
                
                solver::do_move(&mut c, *movement);
                current_moves.push(*movement);
                //println!(
                //    "********************\nCorner: {}, \nEdge: {}, \nUDSortedSlice: {}",
                //    c.corner_permutation, c.phase_two_edge_permutation, c.ud_sorted_slice
                //);
                if phase_two_subgoal(&mut c) || c.edge_permutation != 43954713 {
                    *rubiks = c;

                    acc_final_moves = current_moves;
                    break;
                } else {
                    //println!("Last move: {:?}", *movement);
                    phase_two_search(&mut c, my_depth - 1, &current_moves);
                };
                //println!("MoveList: {:?}", current_moves);
            }
        }
        my_depth = 0;
    }

    acc_final_moves
}

fn phase_two_subgoal(rubiks: &mut physical::Cube) -> bool {
    (rubiks.corner_permutation == 0) && (rubiks.edge_permutation == 0) && (rubiks.ud_slice == 0)
}