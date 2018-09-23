//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

use std::io::{self, Read};

mod facelets;
mod physical;
mod solver;
mod utility;

fn main() {
    let mut not_exit = true;

    println!("Please insert your cube, press Q to exit and H for help: ");
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let option: Result<String, _> = line.trim().parse();
        match option {
            Ok(cube) => {
                if cube.to_ascii_uppercase() == "Q" {
                    break;
                } else if cube.to_ascii_uppercase() == "H" {
                    // Might redo this and make it a better interface in general. @@TODO@@
                    println!("Insert your U, D, L, R, F or B (each repressenting a different colour of the cube.)The first nine values should represent the Upper face, the next the right face, then the front face, down, left and finally back. Each face should describe the top left to bottom right facelets. ")
                } else if cube.len() > 54 {
                    println!("Your input has more facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else if cube.len() < 54 {
                    println!("Your input has less facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else {
                    let face = facelets::Face::new(&cube);
                    let return_code = face.check_if_can_be_solved();
                    match return_code {
                        0 => {
                            let mut my_cube = face.turn_into_cube();
                            solver::complete_search(&mut my_cube);
                        },
                           1 => println!("You don't have 9 facelets of each colour."),
          2 => println!("Not all the edges exist (there may be multiple edges with the same two colours.)"),
          3 => println!("Not all the corners exist (there may be multiple corners with the same three colours.)"),
          4 => println!("Edge and Corner parities aren't equal."),
          5 => println!("The total Edge flip is wrong."),
          6 => println!("The total Corner twist is wrong."),
          _ => panic!("How on earth did you get a different return code????"),
                    }
                }
            }
            Err(_) => println!("Invalid input; try again."),
        }
    }

    let mut test = facelets::Face::new("UUUUUULLLURRURRURRFFFFFFFFFRRRDDDDDDLLDLLDLLDBBBBBBBBB");
    let mut my_cube = test.turn_into_cube();
    solver::complete_search(&mut my_cube);
    println!("Return Code: {}", test.check_if_can_be_solved());
}
