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

use prunning;
use std::cmp;


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
/*
/// Finds the solution of a rubiks cube in under 30 moves. Uses two IDA*
/// searches.
///
/// # Parameters
/// * `rubiks` - A `Cube` struct that we are trying to solve. Is mutable
///     and a reference so other functions can use it from main.
/// # Returns
/// * `&'static str` - Returns move list.
pub fn complete_search(rubiks: &mut physical::Cube) -> Vec<Moves>{
    let tables: prunning::Tables = prunning::Tables::load_tables();
    let mut c = rubiks.clone();
    
    let mut pristine_state_move_list: Vec<Moves> = Vec::new();
    let b: bool;
    let mut max_depth = 25;

    let mut depth = 0;
    while depth < max_depth {
        let (move_list, c, b) = phase_one_search(Vec::new(), c, depth, &tables);
        if b {
            pristine_state_move_list = move_list;
            break;}
        depth = depth + 1;
    }

    print!("Search Done?! Moves: {:?} \n", pristine_state_move_list);
    
    pristine_state_move_list
}

pub fn phase_one_search(moves: Vec<Moves>, rubiks: physical::Cube, depth: usize, tables: &prunning::Tables) -> (Vec<Moves>, physical::Cube, bool){
    if depth == 0 {
        if phase_one_subgoal(rubiks){
            if moves.len()  == 0 {
                return (moves, rubiks, true);
            }
            else{
                let mut last_move = moves[moves.len() - 1];
                if last_move == Moves::R1
                    || last_move == Moves::R3
                    || last_move == Moves::L1
                    || last_move == Moves::L3
                    || last_move == Moves::F1
                    || last_move == Moves::F3
                    || last_move == Moves::B1
                    || last_move == Moves::B3{
                        print!("G1 moves: {:?} \n", moves);
                        return phase_two_start(moves, rubiks, 25 - depth, tables);
                }
            }
        }
    }
    else if depth > 0 {
        if phase_one_prune(rubiks, tables) <= depth {
            for i in PHASE_ONE_MOVE_LIST.iter() {
                let mut move_list = moves.clone();
                if move_list.len() >= 1
                    && (cannot_follow(move_list[move_list.len() - 1]) == *i
                        || opposite_move(move_list[move_list.len() - 1]) == *i) {
                        continue;
                    }
                move_list.push(*i);
                let (m, c, b) = phase_one_search(move_list, do_move(rubiks, *i), depth - 1, tables);
                if b {
                    return (m, c, b);
                }
            }
        }
    }

    return (moves, rubiks, false);
}

fn phase_one_prune(rubiks: physical::Cube, tables: &prunning::Tables) -> usize {
    print!("Phase One Prune: {}\n", 
    cmp::max(tables.udslice_twist_prune.get(rubiks.ud_slice(), rubiks.corner_orientation()) as usize,
             tables.udslice_flip_prune.get(rubiks.ud_slice(), rubiks.edge_orientation()) as usize));
    cmp::max(tables.udslice_twist_prune.get(rubiks.ud_slice(), rubiks.corner_orientation()) as usize,
             tables.udslice_flip_prune.get(rubiks.ud_slice(), rubiks.edge_orientation()) as usize)
}

fn phase_two_start(moves: Vec<Moves>, rubiks: physical::Cube, max_depth: usize, tables: &prunning::Tables) -> (Vec<Moves>, physical::Cube, bool){
    if phase_two_subgoal(rubiks){
        print!("Phase two not required!!, Moves: {:?}\n", moves);
        return (moves, rubiks, true);
    }
    for depth in 0..max_depth{
        let (m, r, b) = phase_two_search(moves.clone(), rubiks, depth, tables);
        if b { return (m, r, b); }
    }

    (moves, rubiks, false)
}

fn phase_two_search(moves: Vec<Moves>, rubiks: physical::Cube, depth: usize, tables: &prunning::Tables) -> (Vec<Moves>, physical::Cube, bool){
    if depth != 0{
        print!("Depth: {}\n", depth);
    }
    if depth == 0 {
        if phase_two_subgoal(rubiks) {
            // Solved!!
            // Update Max depth later.
            print!("Phase Two Moves: {:?}\n", moves);
            return (moves, rubiks, true)
        }
    }
    else if depth > 0 {
        if phase_two_prune(rubiks, tables) <= depth {
            for i in PHASE_TWO_MOVE_LIST.iter() {
                let mut move_list = moves.clone();
                if move_list.len() >= 1
                    && (cannot_follow(move_list[move_list.len() - 1]) == *i
                        || opposite_move(move_list[move_list.len() - 1]) == *i) {
                        continue;
                    }
                move_list.push(*i);
                let (m, c, b) = phase_two_search(move_list, do_move(rubiks, *i), depth - 1, tables);
                if b {
                    return (m, c, b);
                }
            }
        }
    }

    return (moves, rubiks, false)
}

fn phase_two_prune(rubiks: physical::Cube, tables: &prunning::Tables) -> usize{
    cmp::max(tables.edge4_corner_prune.get(rubiks.ud_sorted_slice(), rubiks.corner_permutation()) as usize,
             tables.edge4_edge8_prune.get(rubiks.ud_sorted_slice(), rubiks.ud_slice()) as usize)
}

/// Checks if the conditions for a G1 state cube have been achieved.
///
/// # Parameters
/// * `rubiks` - A `Cube` type object that we are checking for G1 state.
/// # Returns
/// * `bool` - True or false depending if the cube is in a G1 state.
fn phase_one_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_orientation() == 0)
        && (rubiks.edge_orientation() == 0)
        && (rubiks.ud_slice() == 0)
}

/// Checks if the conditions for a pristine state cube have been achieved.
///
/// # Parameters
/// * `rubiks` - A `Cube` type object that we are checking for a pristine state.
/// # Returns
/// * `bool` - True or false depending if the cube is in a pristine state.
fn phase_two_subgoal(rubiks: physical::Cube) -> bool {
    (rubiks.corner_permutation() == 0)
        && (rubiks.phase_two_edge_permutation() == 0)
         && (rubiks.ud_sorted_slice() == 0)
    //true
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

*/
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
const PHASE_TWO_MOVE_LIST: [Moves; 10] = [
    Moves::U1,
    Moves::U2,
    Moves::U3,
    Moves::B2,
    Moves::F2,
    Moves::D1,
    Moves::D2,
    Moves::D3,
    Moves::L2,
    Moves::R2,
];
