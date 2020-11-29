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
//! Console based user interface.

use crate::facelets;
use facelets::IFace;
use crate::solver;

#[allow(dead_code)]
pub fn create_terminal(){
    // Command line
    //let not_exit = true; // Used to be mut
    loop {
        println!("Please insert your cube, press Q to exit and H for help: ");
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let option: Result<String, _> = line.trim().parse();
        match option {
            Ok(cube) => {
                if cube.to_ascii_uppercase() == "Q" {
                    break;
                } else if cube.to_ascii_uppercase() == "H" {
                    // Might redo this and make it a better interface in general. @@TODO@@
                    println!("Insert your U, R, F, D, L, B (each repressenting a different colour of the cube.)The first nine values should represent the Upper face, the next the right face, then the front face, down, left, and finally back. Each face should describe the top left to bottom right facelets. ")
                } else if cube.len() > 54 {
                    println!("Your input has more facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else if cube.len() < 54 {
                    println!("Your input has less facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else {
                    println!("Valid input; looking for moves now!");
                    let face = facelets::Face::new(&cube);
                    let (msg, success) = face.return_code_matcher();
                    print!("{}", msg);
                    if success {
                        let mut c_cube = face.turn_into_cube();
                        print!("{:?}", solver::complete_search(&mut c_cube))
                    }
                }
            }
            Err(_) => println!("Invalid input; try again."),
        }
    }
    println!("Goodbye!");
}
