//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************
//!
//! Module that deals with the solving of the rubiks cube. This is done in two
//! parts, each focusing on a different mathematical group to solve. Both phases
//! use the same implamentation of IDA*, with different depths and goals.

use physical;

/// All the avaliable moves you can perfom on a rubiks cube. x3 is an
/// anti-clockwise movement.
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

/// Finds the solution of a rubiks cube in under 30 moves. Uses two IDA*
/// searches.
///
/// # Parameters
/// * `rubiks` - A `Cube` struct that we are trying to solve. Is mutable
///     and a reference so other functions can use it from main.
/// # Returns
/// * `&'static str` - Returns move list.
pub fn complete_search(rubiks: &mut physical::Cube) -> String{
    let a = Vec::new();
    //let b = Vec::new();
    let mut c = rubiks.clone();
    //println!("We got to the cloning.");///RM
    /*
    let g1_state_move_list = search(
        &mut c,
        a,
        MAX_PHASE_ONE_DEPTH,
        phase_one_subgoal,
        &PHASE_ONE_MOVE_LIST,
    );
    return format!("Phase1 move list: {:?}", &g1_state_move_list[..]);
     */
    return "Search function".to_string();
    //println!("We State Move list complete.");///RM
    /*
    let pristine_state_move_list = search(
        &mut c,
        b,
        MAX_PHASE_TWO_DEPTH,
        phase_two_subgoal,
        &PHASE_TWO_MOVE_LIST,
    );

    return format!(
        "Move list: {:?}",
        [&g1_state_move_list[..], &pristine_state_move_list[..]].concat()
    );*/
}

/// Checks if the conditions for a G1 state cube have been achieved.
///
/// # Parameters
/// * `rubiks` - A `Cube` type object that we are checking for G1 state.
/// # Returns
/// * `bool` - True or false depending if the cube is in a G1 state.
fn phase_one_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_orientation == 0) && (rubiks.edge_orientation == 0) && (rubiks.ud_slice == 0)
}

/// Checks if the conditions for a pristine state cube have been achieved.
///
/// # Parameters
/// * `rubiks` - A `Cube` type object that we are checking for a pristine state.
/// # Returns
/// * `bool` - True or false depending if the cube is in a pristine state.
fn phase_two_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_permutation == 0)
        && (rubiks.phase_two_edge_permutation == 0)
        && (rubiks.ud_sorted_slice == 0)
}

/// Basic IDA* search using the movements of the rubiks as a faux tree.
///
/// # Parameters
/// * `rubiks` - A mutable reference to the `Cube` we are solving.
/// * `move_list` - A `Vec<Moves>` of the current `Moves` done so far.
/// * `max_depth` - A `usize` describing the maximum length the search should
///     bother with.
/// * `subgoal` - A function that tests the a `Cube` for a solution state.
/// * `whole_move_list` - What rubiks cube moves are valid in the current phase
///     of the search.
/// # Returns
/// * `Vec<Moves>` - The moves required to get from the input cube to a solution
///     state.
pub fn search(
    rubiks: &mut physical::Cube,
    move_list: Vec<Moves>,
    max_depth: usize,
    subgoal: fn(physical::Cube) -> bool,
    whole_move_list: &[Moves],
) -> Vec<Moves> {
    let mut results: bool = false;
    let mut c: physical::Cube = rubiks.clone();
    let mut solution: Vec<Moves> = Vec::new();
    if subgoal(rubiks.clone()) {
        results = true;
    }
    for i in 0..max_depth {
        //println!("{} out of {}", i, max_depth);
        tree_search(
            rubiks,
            &mut c,
            i,
            move_list.clone(),
            &mut results,
            &mut solution,
            whole_move_list,
            subgoal,
        );
        if results == true {
            break;
        };
    }
    solution
}

/// The actual tree search used in the IDA* algorithim.
///
/// # Parameters
/// * `rubiks` - A `Cube` that we are trying to solve. Note this is before any
///     recursion occurs and this only changes if a solution is found.
/// * `dummy_rubiks` - A `Cube` we pass between recursions that reflects the
///     current state of the cube at any point in the tree. Upon a solution
///     being found we set `rubiks` to this.
/// * `move_list` - The current moves done going down the tree.
/// * `found` - Mutable `bool` signialling if a solutions has been found yet
///     or not. If a solution has been found we break out of any loops and
///     recursions.
/// * `final_list` - The final `Vec<Moves>` required to get the original `Cube`
///     to the solution state.
/// * `whole_move_list` - The current set of `Moves` that can be taken.
/// * `subgoal` - A function that tests to see if the correct permutation of
///     `Moves` have been performed to solve the `Cube`.
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
                let last_move: Moves;
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
                    current_list.push(*movement);

                    if subgoal(c) {
                        *found = true;
                        *final_list = current_list.clone();
                        *rubiks = c.clone();
                        break;
                    } else {
                        tree_search(
                            &mut *rubiks,
                            &mut c,
                            depth - 1,
                            current_list.clone(),
                            &mut *found,
                            &mut *final_list,
                            whole_move_list,
                            subgoal,
                        );
                    }
                }
            }
        }
    }
}

/// Pattern matching function that inputs a `Cube` and returns a `Cube` that has
/// had the required move applied to it.
///
/// # Parameters
/// * `rubiks` - The `Cube` to apply the `movement` to.
/// * `movement` - A `Moves` to apply to `rubiks`.
/// # Returns
/// * `Cube` - A `Cube` with the `movement` applied to it.
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
        Moves::NONE => {}
    }
    rubiks
}

/// A pattern matching function which dictates what `Moves` are mathematically
/// equal according to our group theory definitions of the cube.
///
/// # Parameters
/// * `movement` - A `Moves` to find the opposite equal of.
/// # Returns
/// * `Moves` - The mathematical equal of `movement`.
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

/// A pattern matching function that dictates which `Moves` should not follow
/// another as they essentially repetitions.
///
/// # Parameters
/// * `movement` - The `Moves` of which to find the matching `Moves`.
/// # Returns
/// * `Moves` - The `Moves` the shouldn't follow `movement`
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

///*****************************************************************************
///* Constant values.
///****************************************************************************

const MAX_PHASE_ONE_DEPTH: usize = 21;//18;
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
const MAX_PHASE_TWO_DEPTH: usize = 10;//12;
const PHASE_TWO_MOVE_LIST: [Moves; 6] = [
    Moves::U1,
    //Moves::U2,
    //Moves::U3,
    Moves::B2,
    Moves::F2,
    Moves::D1,
    //Moves::D2,
    //Moves::D3,
    Moves::L2,
    Moves::R2,
];
