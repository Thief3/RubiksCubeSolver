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

use std::cmp;
use crate::cubes::coord_cube::{ CoordCube, Moves, MOVE_LIST, PHASE_TWO_MOVE_LIST };
use crate::cubes::face_cube::FaceCube;
use crate::cubes::cubie_cube::CubieCube;
use crate::defs::corner_cubies::Corner;
use crate::defs::edge_cubies::Edge;
use crate::defs::facelets::{ EDGE_LIST };

use std::thread;
pub fn solve(cc: CubieCube){
    cross_solve(cc);
}

const CROSS_EDGES: [Edge; 4] = [Edge::UR, Edge::UF, Edge::UL, Edge::UB];

pub fn cross_solve(c: CubieCube) -> CubieCube{
    let mut cc = c.clone();
    let mut moves: Vec<usize> = Vec::new();
    println!("Cross Solve started.");
    // Iterate through top edges.
    for edge in 0..12 {
        let perm = cc.edge_permutation[edge];
        if CROSS_EDGES.contains(&perm){

            let ori = cc.edge_orientation[edge];
                
            // Perm right, Orientation Wrong
            if perm == EDGE_LIST[edge] && ori != 0 {
                let (cc, new_moves) = cross_right_perm_wrong_ori(cc.clone(), &perm);
                moves.extend(&new_moves);
                println!("Perm now: {:?}, Edge now: {:?}", cc.edge_permutation[edge], cc.edge_orientation[edge]);
                //break;
            }           
        }
    }
    
    for m in moves {
        let out = match m {
            0 => "U",
            1 => "R",
            2 => "F",
            3 => "D",
            4 => "L",
            5 => "B",
            _ => panic!("How did you get move: {}", m),
        };

        print!("{} ", out);
    }
    print!("\n");
    cc
}

pub fn cross_layer_goal(cc: CubieCube) -> bool {
    let mut b = true;

    for i in 0..4 {
        if cc.edge_permutation[i] != CROSS_EDGES[i] {
            b = false;
            break;
        }
        if cc.edge_orientation[i] != 0 {
            b = false;
            break;
        }
    }

    b
}

pub fn cross_right_perm_wrong_ori(c: CubieCube, perm: &Edge) -> (CubieCube, Vec<usize>)  {
    let mut cc = c.clone();

    // Moves F R3 D3 R F2 but rotated depending on which face should be the front.
    let move_set = match perm {
        Edge::UF => vec![2, 1, 1, 1, 3, 3, 3, 1, 2, 2],
        Edge::UR => vec![1, 5, 5, 5, 3, 3, 3, 5, 1, 1],
        Edge::UB => vec![5, 4, 4, 4, 3, 3, 3, 4, 5, 5],
        Edge::UL => vec![4, 2, 2, 2, 3, 3, 3, 2, 4, 4],
        _ => panic!("How has edge {:?}, got this far?", perm)
    };
    
    for i in &move_set{
        cc.movement_u(*i)
    }

    println!("Cross right wrong done once!");
    (cc, move_set)
}

pub fn first_two_layers_solve(){
    
}

pub fn first_layer_goal(cc: CubieCube) -> bool {
    let mut b = true;
    let checks = [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR];

    for i in 0..4{
        if cc.corner_permutation[i] != checks[i]{
            b = false;
        }
    }

    b
}

pub fn orientation_last_layer_solve(){
    
}

pub fn permutation_last_layer_solve(){
    
}
