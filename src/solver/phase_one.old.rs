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
    rubiks: &mut physical::Cube,
    depth: i32,
    move_list: &Vec<solver::Moves>,
) -> Vec<solver::Moves> {
    let mut final_moves = move_list.clone();
    let mut acc_final_moves = move_list.clone();
    //println!("Final Moves: {:?}", final_moves);
    let mut my_depth = depth;
    //println!("Depth: {}", my_depth);
    while (!phase_one_subgoal(rubiks) && my_depth > 0) {
        for movement in PHASE_ONE_MOVE_LIST.iter() {
            let unwrapped = final_moves.last().unwrap();
            println!("{:?}", unwrapped);
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
                //    "Corner: {}, \nEdge: {}, \nUDSlice: {}",
                //    c.corner_orientation, c.edge_orientation, c.ud_slice
                //);
                if phase_one_subgoal(&mut c) {
                    *rubiks = c;

                    acc_final_moves = current_moves;
                    break;
                } else {
                    //println!("Last move: {:?}", *movement);
                    phase_one_search(&mut c, my_depth - 1, &current_moves);
                };
                //println!("MoveList: {:?}", current_moves);
            }
        }
        my_depth = 0;
    }

    acc_final_moves
}

fn phase_one_subgoal(rubiks: &mut physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0) && (rubiks.ud_slice == 0)
}
