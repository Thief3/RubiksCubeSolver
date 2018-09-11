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

static mut DONE: bool = false;

pub fn phase_one_search(
    rubiks: &mut physical::Cube,
    depth: i32,
    move_list: &Vec<solver::Moves>) -> bool{

    let mut a = false;
    //println!("{:?}", move_list);
    if phase_one_subgoal(rubiks) {
        println!("Solved.");
        
        a = true;
    } else if depth > 0 && !phase_one_subgoal(rubiks) {
        let mut final_moves = move_list.clone();
        for movement in PHASE_ONE_MOVE_LIST.iter(){
            let unwrapped = final_moves.last().unwrap();
            //println!("{:?}", unwrapped);
            if final_moves.len() == 0
                || (*unwrapped != *movement &&
                    *unwrapped == solver::Moves::NONE || (
                    *unwrapped != solver::opposite_move(*unwrapped) &&
                    *unwrapped != solver::cannot_follow(*unwrapped))) &&
                !a {
                let mut c = rubiks.clone();
                let mut current_move = final_moves.clone();
                current_move.push(*movement);

                solver::do_move(&mut c, *movement);
                a = phase_one_search(&mut c, depth - 1, &current_move);
            }
        }
    } else{
        //println!("For fucks sake.");
    }
    a
}

fn phase_one_subgoal(rubiks: &mut physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0)  && (rubiks.ud_slice == 0)
}