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
use crate::cubes::coord_cube::{ CoordCube, Moves, MOVE_LIST };
use crate::cubes::face_cube::FaceCube;

#[derive(Clone)]
pub struct Solver {
    original_cc: CoordCube,
    cc: CoordCube,
    max_depth: isize,
    tables: prunning::Tables,

    axis: Vec<isize>,
    power: Vec<isize>,
    twist: Vec<isize>,
    flip: Vec<isize>,
    udslice: Vec<isize>,
    corner: Vec<isize>,
    edge4: Vec<isize>,
    edge8: Vec<isize>,

    min_dist_1: Vec<isize>,
    min_dist_2: Vec<isize>
}

impl Solver{
    pub fn new(cc: CoordCube, max_depth: usize) -> Solver{
        let mut sol = Solver{
            original_cc: cc.clone(),
            cc: cc.clone(),
            max_depth: max_depth as isize,
            tables: cc.tables.clone(),

            axis: vec![0; max_depth],
            power: vec![0; max_depth],
            twist: vec![0; max_depth],
            flip: vec![0; max_depth],
            udslice: vec![0; max_depth],
            corner: vec![0; max_depth],
            edge4: vec![0; max_depth],
            edge8: vec![0; max_depth],

            min_dist_1: vec![0; max_depth],
            min_dist_2: vec![0; max_depth],
        };

        sol.twist[0] = cc.twist as isize;
        sol.flip[0] = cc.flip as isize;
        sol.udslice[0] = cc.udslice as isize;
        sol.corner[0] = cc.corner as isize;
        sol.edge4[0] = cc.edge4 as isize;
        sol.edge8[0] = cc.edge8 as isize;
        sol.min_dist_1[0] = sol.clone().phase_one_cost(0);

        sol
    }
    
    pub fn solve(&mut self) -> String{
        let mut s: String = "No Solutions found in depth range.".to_string();
        for depth in 0..self.max_depth {
            let n = self.phase_one_search(0, depth);
            if n >=0 {
                println!("Solved: {}", n);
                s = self.clone().solution_to_string(n as usize);
                println!("Move set: {}", s);
            }
        }

        s
    }

    pub fn phase_two_init(&mut self, n: isize) -> isize{
        let mut cc = self.original_cc.clone();
        for i in 0..(n as usize){
            // Power shouldn't ever be -1;
            for _j in 0..(self.power[i] as usize){
                if(self.axis[i] >= 0){
                    cc.movement(MOVE_LIST[self.axis[i] as usize]);
                }
            }
        }
        self.edge4[(n as usize)] = cc.edge4 as isize;
        self.edge8[(n as usize)] = cc.edge8 as isize;
        self.corner[(n as usize)] = cc.corner as isize;
        self.min_dist_2[(n as usize)] = self.clone().phase_two_cost(n);
        for depth in 0..(self.max_depth - n){
            let m = self.phase_two_search(n, depth);
            if m >= 0 {
                return m;
            }
        }
        return -1;
    }
    
    pub fn phase_one_cost(self, n: isize) -> isize{
        std::cmp::max(
            self.cc.tables.udslice_twist_prune.get(
                self.udslice[(n as usize)],
                self.twist[(n as usize)]),
            self.cc.tables.udslice_flip_prune.get(
                self.udslice[(n as usize)],
                self.flip[(n as usize)])
        )
    }

    pub fn phase_two_cost(self, n: isize) -> isize{
        std::cmp::max(
            self.cc.tables.edge4_corner_prune.get(
                self.edge4[(n as usize)],
                self.corner[(n as usize)]),
            self.cc.tables.edge4_edge8_prune.get(
                self.edge4[(n as usize)],
                self.edge8[(n as usize)])
        )
    }

    pub fn phase_one_search(&mut self, n: isize, depth: isize) -> isize{
        //println!("Phase 1 search starting\n");
        if self.min_dist_1[(n as usize)] == 0{
            println!("Phase two started.");
            return 1; //self.phase_two_init(n);
        }
        else if self.min_dist_1[(n as usize)] <= depth{
            //println!("Trying moves.");
            for i in 0..6{
                // Don't do consecutive moves  of the same type.
                if n > 0 && [i, i + 1, i + 2].contains(&self.axis[(n as usize) - 1]){
                    continue;
                }
                
                for j in 1..4{
                    self.axis[(n as usize)] = i;
                    self.power[(n as usize)] = j;
                    let mv = 3 * i + j - 1;

                    self.twist[(n as usize) + 1] = self.tables.twist_move[self.twist[(n as usize)] as usize][mv as usize];
                    self.flip[(n as usize) + 1] = self.tables.flip_move[self.flip[(n as usize)] as usize][mv as usize];
                    self.udslice[(n as usize) + 1] = self.tables.udslice_move[self.udslice[(n as usize)] as usize][mv as usize];
                    self.min_dist_1[(n as usize) + 1] = self.clone().phase_one_cost(n + 1);

                    let m = self.phase_one_search(n + 1, depth - 1);
                    if m >= 0{
                        return m;
                    }
                }
            }
        }

        return -1;
    }

    pub fn phase_two_search(&mut self, n: isize, depth: isize) -> isize {
        println!("Min_dist_2: {}", self.min_dist_2[(n as usize)]);
        if self.min_dist_2[(n as usize)] == 0{
            return n as isize
        }
        else if self.min_dist_2[(n as usize)] <= depth {
            for i in 0..6{
                if n > 0 && [i, i + 1, i + 2].contains(&self.axis[(n as usize) - 1]){
                    continue;
                }
                for j in 1..4{
                    // Only do moves R, F , L , B
                    if [1, 2, 4, 5].contains(&i) && j != 2{
                        continue;
                    }
                    self.axis[(n as usize)] = i;
                    self.power[(n as usize)] = j;
                    let mv = 3 * i + j - 1;

                    self.edge4[(n as usize) + 1] = self.tables.edge4_move[self.edge4[(n as usize)] as usize][mv as usize];
                    self.edge8[(n as usize) + 1] = self.tables.edge8_move[self.edge8[(n as usize)] as usize][mv as usize];
                    self.corner[(n as usize) + 1] = self.tables.corner_move[self.corner[(n as usize)] as usize][mv as usize];
                    self.min_dist_2[(n as usize) + 1] = self.clone().phase_two_cost(n + 1);

                    let m = self.phase_two_search(n + 1, depth - 1);
                    if m >= 0{
                        return m;
                    }
                    
                }
            }
        }

        return -1;
    }

    pub fn solution_to_string(self, length: usize) -> String{
        let mut moves: Vec<String> = Vec::new();
        for i in 0..length {
            let s1 = match self.axis[i] {
                0 => 'U',
                1 => 'R',
                2 => 'F',
                3 => 'D',
                4 => 'L',
                5 => 'B',
                _ => panic!("There shouldn't be a number higher than 5 in axis?")
            };

            let s2 = match self.power[i] {
                1 => "",
                2 => "2",
                3 => "'",
                _ => panic!("Unknown value in power?")
            };

            moves.push(format!("{}{}", s1, s2));
        }

        let s = moves.join(" ");
        s
    }
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
        Moves::R3 => Moves::L3
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
        Moves::R3 => Moves::R2
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
