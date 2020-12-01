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
//! Module that uses move and prunning tables to greatly reduce the time needed
//! to solve a cube.
//! Move tables are used for updating coordinate representation of cube when a
//! particular move is applied.
//! Pruning tables are used to obtain lower bounds for the number of moves
//! required to reach a solution given a particular pair of coordinates.

use std::fs::File;
use io::BufReader;

#[derive(Clone, Debug)]
pub struct PruningTable {
    table:  Vec<isize>,
    stride: usize
}

impl PruningTable{
    pub fn get(&self, x: isize, y: isize) -> isize{
        self.table[x as usize * self.stride + y as usize]
    }
}

// 3^7 possible corner orientations
const TWIST: usize = 2187;
// 2^11 possible edge flips
const FLIP: usize = 2048;
// 12C4 possible positions of FR, FL, BL, BR
const UDSLICE: usize = 495;
// 4! possible permutations of FR, FL, BL, BR
const EDGE4: usize = 24;
// 8! possible permutations of UR, UF, UL, UB, DR, DF, DL, DB in phase two
const EDGE8: usize = 40320;
// 8! possible permutations of the corners
const CORNER: usize = 40320;
// 12! possible permutations of all edges
const EDGE: usize = 479001600;
// 6*3 possible moves
const MOVES: usize = 18;

#[derive(Serialize, Deserialize, Debug)]
struct JsonTables {
    // Move
    #[serde(rename = "twist_move")]
    pub twist_move: Vec<Vec<isize>>,
    #[serde(rename = "flip_move")]
    pub flip_move: Vec<Vec<isize>>,
    #[serde(rename = "udslice_move")]
    pub udslice_move: Vec<Vec<isize>>,
    #[serde(rename = "edge4_move")]
    pub edge4_move: Vec<Vec<isize>>,
    #[serde(rename = "edge8_move")]
    pub edge8_move: Vec<Vec<isize>>,
    #[serde(rename = "corner_move")]
    pub corner_move: Vec<Vec<isize>>,

    // Prunning
    #[serde(rename = "udslice_twist_prune")]
    pub udslice_twist_prune: Vec<isize>,
    #[serde(rename = "udslice_flip_prune")]
    pub udslice_flip_prune: Vec<isize>,
    #[serde(rename = "edge4_edge8_prune")]
    pub edge4_edge8_prune: Vec<isize>,
    #[serde(rename = "edge4_corner_prune")]
    pub edge4_corner_prune: Vec<isize>,
}

#[derive(Clone, Debug)]
pub struct Tables {
    // Move
    pub twist_move: Vec<Vec<isize>>,
    pub flip_move: Vec<Vec<isize>>,
    pub udslice_move: Vec<Vec<isize>>,
    pub edge4_move: Vec<Vec<isize>>,
    pub edge8_move: Vec<Vec<isize>>,
    pub corner_move: Vec<Vec<isize>>,

    // Prunning
    pub udslice_twist_prune: PruningTable,
    pub udslice_flip_prune: PruningTable,
    pub edge4_edge8_prune: PruningTable,
    pub edge4_corner_prune: PruningTable,
}

impl Tables {  
    pub fn load_tables() -> Tables{
        // Yolo if this doesn't exist.
        let file = File::open("./assets/tables.json").expect("No table.json");
        let reader = BufReader::new(file);

        let json: JsonTables = serde_json::from_reader(reader).expect("IDK");
        Tables{
            twist_move: (json.twist_move),
            flip_move: (json.flip_move),
            udslice_move: (json.udslice_move),
            edge4_move: (json.edge4_move),
            edge8_move: (json.edge8_move),
            corner_move: (json.corner_move),
            
            udslice_twist_prune: PruningTable{
                table: json.udslice_twist_prune,
                stride: TWIST
            },
            udslice_flip_prune: PruningTable{
                table: json.udslice_flip_prune,
                stride: FLIP
            },
            edge4_edge8_prune: PruningTable{
                table: json.edge4_edge8_prune,
                stride: EDGE8
            },
            edge4_corner_prune: PruningTable{
                table: json.edge4_corner_prune,
                stride: CORNER
            },
        }
    }
    
    pub fn make_twist_table(){
        
    }

    pub fn make_flip_table(){
        
    }

    pub fn make_ud_slice_table(){
        
    }

    pub fn make_edge4_table(){
        
    }

    pub fn make_edge8_table(){
        
    }

    pub fn make_corner_table(){
        
    }

    pub fn make_ud_slice_twist_prune(){
        
    }

    pub fn make_ud_slice_flip_prune(){
        
    }

    pub fn make_edge4_edge8_prune(){
        
    }

    pub fn make_edge4_corner_prune(){
        
    }
}
