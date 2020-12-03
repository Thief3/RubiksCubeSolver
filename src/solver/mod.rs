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
    let mut cc_1 = c.clone();
    let mut moves: Vec<usize> = Vec::new();
    let mut new_moves: Vec<usize> = Vec::new();
    println!("Cross Solve started.");
    //while !cross_layer_goal(cc){
        println!("Loop cc perm: {:?}", cc.edge_permutation);
    // Iterate through top edges.
        for edge in 0..12 {
            let mut perm = cc.edge_permutation[edge];
            // Check if its one of the cross edges.
            if CROSS_EDGES.contains(&perm){
                
                let mut ori = cc.edge_orientation[edge];
                
                // Perm right, Orientation Wrong
                if perm == EDGE_LIST[edge] && ori != 0 {
                    let (cc_1, new_moves) = cross_right_perm_wrong_ori(cc.clone(), &perm);
                    moves.extend(&new_moves);
                }
                
                perm = cc.edge_permutation[edge];
                ori = cc.edge_orientation[edge];
                
                // If the edge is in the middle layer
                if [8, 9, 10, 11].contains(&edge){
                    let (cc_1, new_moves) = cross_middle_layer(cc.clone(), &perm, edge);
                    moves.extend(&new_moves);
                }

                // Hidden ones.
                else if [4, 5, 6, 7].contains(&edge){
                    let (cc_1, new_moves) = cross_hidden(cc.clone(), &perm);
                    moves.extend(&new_moves);
                }
            }
        }

        let mut b = true;
        for i in 0..4 {
            if cc_1.edge_permutation[i] != CROSS_EDGES[i]{
                b = false;
                break;
            }
        }
        if b {
            let (cc_1, new_moves) = cross_vertical_align(cc.clone());
            moves.extend(&new_moves);    
        }

        cc = cc_1.clone();
        //println!("Moves are: {:?}", moves_to_string(moves.clone()));
    //}
    
    println!("Moves are: {:?}", moves_to_string(moves));  
    cc
}

pub fn moves_to_string(moves: Vec<usize>) -> String {
    let mut out: String = "".to_owned();
    let mut i = 0;
    while i < moves.len(){
        let s = match moves[i] {
            0 => "U",
            1 => "R",
            2 => "F",
            3 => "D",
            4 => "L",
            5 => "B",
            _ => panic!("How did you get move: {}", moves[i]),
        };
        let mut count = 1;
        if i + 1 < moves.len()
            && moves[i + 1] == moves[i] {
                count = count + 1;
                if i + 2 < moves.len()
                    && moves[i + 1] == moves[i]{
                        count = count + 1;
                }
            }
        
        out.push_str(s);

        if count != 1{
            out.push_str(&count.to_string());
        }

        out.push_str(" ");
        i = i + count;
    }
    
    out
}

pub fn cross_layer_goal(cc: CubieCube) -> bool {
    let mut b = true;

    //println!("Permutation: {:?}", cc.edge_permutation);
    
    for i in 0..4 {
        //println!("Our perm: {:?} vs should be: {:?}", cc.edge_permutation[i], CROSS_EDGES[i]);
        //println!("Our ori: {:?}", cc.edge_orientation[i]);
        if cc.edge_permutation[i] != CROSS_EDGES[i]
            || cc.edge_orientation[i] != 0{
                b = false;
                break;
        }
    }

    b
}

pub fn cross_vertical_align(c: CubieCube) -> (CubieCube, Vec<usize>){
    let mut cc = c.clone();

    let mut move_set: Vec<usize> = vec![];

    let mut count = 0;
    let mut incorrect_edge = cc.edge_permutation[0];
    for i in 0..4 {
        if cc.edge_permutation[i] != CROSS_EDGES[i] {
            count = count + 1;
            if count == 1 {
                incorrect_edge = cc.edge_permutation[i];
            }
        }
    }

    if count == 2 {
        // Opposite edges are wrong
        if (cc.edge_permutation[0] != CROSS_EDGES[0] && cc.edge_permutation[2] != CROSS_EDGES[2])
            || (cc.edge_permutation[1] != CROSS_EDGES[1] && cc.edge_permutation[3] != CROSS_EDGES[3]){

                // Moves R2 L2 U2 R2 L2  but rotated depending on which face should be the front.
                move_set = match incorrect_edge {
                    Edge::UF => vec![1, 1, 4, 4, 0, 0, 1, 1, 4, 4],
                    Edge::UR => vec![5, 5, 2, 2, 0, 0, 5, 5, 2, 2],
                    Edge::UB => vec![4, 4, 1, 1, 0, 0, 4, 4, 1, 1],
                    Edge::UL => vec![2, 2, 5, 5, 0, 0, 2, 2, 5, 5],
                    _ => panic!("How has edge {:?}, got this far?", incorrect_edge)
                };
                
                for i in &move_set{
                    cc.movement_u(*i);
                    //move_set.push(*i);
                }
            }
        // Consecutive edges are wrong
        else {
            // Moves F2 D R2 D3 F2 but rotated depending on which face should be the front.
            move_set = match incorrect_edge {
                Edge::UF => vec![2, 2, 3, 1, 1, 3, 3, 3, 2, 2],
                Edge::UR => vec![1, 1, 3, 5, 5, 3, 3, 3, 2, 2],
                Edge::UB => vec![5, 5, 3, 4, 4, 3, 3, 3, 5, 5],
                Edge::UL => vec![4, 4, 3, 2, 2, 3, 3, 3, 4, 4],
                _ => panic!("How has edge {:?}, got this far?", incorrect_edge)
            };
            
            for i in &move_set{
                cc.movement_u(*i);
                //move_set.push(*i);
            }
        }
    }
    else if count == 3 {
        // Moves R B3 R3 B2 but rotated depending on which face should be the front.
        move_set = match incorrect_edge {
            Edge::UF => vec![2, 2, 1, 1, 1, 2, 2, 2, 1, 1],
            Edge::UR => vec![1, 1, 5, 5, 5, 1, 1, 1, 5, 5],
            Edge::UB => vec![5, 5, 4, 4, 4, 5, 5, 5, 4, 4],
            Edge::UL => vec![4, 4, 2, 2, 2, 4, 4, 4, 2, 2],
            _ => panic!("How has edge {:?}, got this far?", incorrect_edge)
        };
        
        for i in &move_set{
            cc.movement_u(*i);
            //move_set.push(*i);
        }
    }
    // Rotate until things line up.
    else if count == 4 {
        println!("I Activate");
        while(cc.edge_permutation[0] != CROSS_EDGES[0]){
            println!("Yas");
            move_set.push(0);
            cc.movement_u(0);
        }
    }

    (cc, move_set)
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

    //println!("Cross right wrong done once!");
    (cc, move_set)
}

pub fn cross_middle_layer(c: CubieCube, perm: &Edge, e: usize) -> (CubieCube, Vec<usize>){
    let mut cc = c.clone();
    let mut move_set: Vec<usize> = vec![];

    // On the right
    if e == 8 || e == 11{
        // Moves R3, D3, R, F2 but rotated depending on which face should be the front.
        move_set = match perm {
            Edge::UF => vec![1, 1, 1, 3, 3, 3, 1, 2, 2],
            Edge::UR => vec![5, 5, 5, 3, 3, 3, 5, 1, 1],
            Edge::UB => vec![4, 4, 4, 3, 3, 3, 4, 5, 5],
            Edge::UL => vec![2, 2, 2, 3, 3, 3, 2, 4, 4],
            _ => panic!("How has edge {:?}, got this far?", perm)
        };
    }
    else {
        // Moves L3, D3, L, F2 but rotated depending on which face should be the front.
        move_set = match perm {
            Edge::UF => vec![4, 4, 4, 3, 3, 3, 4, 2, 2],
            Edge::UR => vec![2, 2, 2, 3, 3, 3, 2, 1, 1],
            Edge::UB => vec![1, 1, 1, 3, 3, 3, 1, 5, 5],
            Edge::UL => vec![5, 5, 5, 3, 3, 3, 5, 4, 4],
            _ => panic!("How has edge {:?}, got this far?", perm)
        };
    }
    


    for i in &move_set{
        cc.movement_u(*i)
    }

    (cc, move_set)
}

pub fn cross_hidden(c: CubieCube, perm: &Edge) -> (CubieCube, Vec<usize>){
    let mut cc = c.clone();
    // Moves F3, R3, D3, R1, F2 but rotated depending on which face should be the front.
    let move_set = match perm {
        Edge::UF => vec![2, 2, 2, 1, 1, 1, 3, 3, 3, 1, 2, 2],
        Edge::UR => vec![1, 1, 1, 5, 5, 5, 3, 3, 3, 5, 1, 1],
        Edge::UB => vec![5, 5, 5, 4, 4, 4, 3, 3, 3, 4, 5, 5],
        Edge::UL => vec![4, 4, 4, 2, 2, 2, 3, 3, 3, 2, 4, 4],
        _ => panic!("How has edge {:?}, got this far?", perm)
    };

    for i in &move_set{
        cc.movement_u(*i)
    }

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
