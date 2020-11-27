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

#[macro_use]
extern crate imgui;
#[macro_use]
extern crate glium;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
extern crate clipboard;

use imgui::*;

mod facelets;
mod physical;
mod solver;
mod utility;
mod ui_support;

pub enum Color {
    White,
    Red,
    Blue,
    Orange,
    Green,
    Yellow
}
impl Color {
    fn get_vec(&self) -> [f32; 4] {
        match self {
            Self::White  => [1.0, 1.0, 1.0, 1.0],
            Self::Red    => [1.0, 0.0, 0.0, 1.0],
            Self::Blue   => [0.0, 0.0, 1.0, 1.0],
            Self::Orange => [1.0, 0.64, 0.0, 1.0],
            Self::Green  => [0.0, 1.0, 0.0, 1.0],
            Self::Yellow => [1.0, 1.0, 0.0, 1.0],
            
        }
    }

    fn get_facelet(&self) -> facelets::Facelets{
        match self {
            Self::White  => facelets::Facelets::U,
            Self::Red    => facelets::Facelets::F,
            Self::Blue   => facelets::Facelets::R,
            Self::Orange => facelets::Facelets::B,
            Self::Green  => facelets::Facelets::L,
            Self::Yellow => facelets::Facelets::D,
        }        
    }
    fn get_char(&self) -> char{
        match self {
            Self::White  => 'u',
            Self::Red    => 'f',
            Self::Blue   => 'r',
            Self::Orange => 'b',
            Self::Green  => 'l',
            Self::Yellow => 'd',
        }        
    }
}

fn convert_color_rubiks_to_chars(rubiks: [Color; 54]) -> facelets::RubiksChar{
    let mut a: facelets::RubiksChar = [' '; 54];
    for i in 0..54{
        a[i] = rubiks[i].get_char();
    }
    return a;
}

fn convert_color_rubiks_to_facelets(rubiks: [Color; 54]) -> facelets::RubiksFacelets{
    let mut a: facelets::RubiksFacelets = [facelets::Facelets::U; 54];
    for i in 0..54{
        a[i] = rubiks[i].get_facelet();
    }
    return a;
}

fn create_window(){
    let mut state = State{
        col: [1.0,0.0,0.0,1.0],
        switch: [0.0,1.0,0.0,1.0],
        colors: [
            Color::White.get_vec(),
            Color::Red.get_vec(),
            Color::Blue.get_vec(),
            Color::Orange.get_vec(),
            Color::Green.get_vec(),
            Color::Yellow.get_vec(),
        ],
        current: Color::White.get_vec(),
        rubiks: [[0.0, 0.0, 0.0, 0.0]; 54],
        notify_text: "",
    };

    for i in 0..6{
        for j in 0..9 {
            state.rubiks[i * 9 + j] = state.colors[i];
        }
    }
    
    let system = ui_support::init(file!());
    system.main_loop(move |run, ui| {
        rubiks_cube_flat(ui, &mut state);
    });
}

/*
fn imgui_str_dynamic(x: String) -> &ImStr{
    return unsafe { ImStr::from_utf8_with_nul_unchecked(format!("{}\0", x).as_bytes()) }
}*/

macro_rules! ig_dynamic_str {
    ($x:expr) => {
        unsafe { ImStr::from_utf8_with_nul_unchecked(format!("{}\0", $x).as_bytes()) }
    }
}

macro_rules! ig_make_label {
    ( $x:expr, $y:expr ) => {
        ig_dynamic_str!(format!("{}::{}", $x, $y))
    }
}

fn row_buttons(ui: &Ui, width: i32, row: i32, state: &mut State){
    for x in 0..width{
        if ColorButton::new(ig_make_label!("Rubiks", (row + x).to_string()),
                            state.rubiks[(row + x) as usize])
            .size([30.0,30.0])
            .tooltip(false)
            .build(ui){
                state.rubiks[(row + x) as usize] = state.current;
                state.notify_text = "Facelet Clicked";
            }
        ui.same_line_with_spacing(0.0, 5.0);
    }
    
    ui.new_line();
}

fn block_buttons(ui: &Ui, width: i32, height: i32, block: i32, state: &mut State){
    unsafe{
        sys::igBeginGroup();
    }
    for y in 0..height{
        // 0 * 9 + 0 * 3 = 0
        // 1 * 9 + 0 * 3 = 9
        row_buttons(ui, width, block * (width * height) + y * width, state);
    }
    unsafe{
        sys::igEndGroup();
    }
}

fn rubiks_cube_flat(ui: &Ui, state: &mut State) {
    let w = Window::new(im_str!("Example 1: Basics"))
        .size([700.0, 550.0], Condition::FirstUseEver)
        .position([20.0, 140.0], Condition::FirstUseEver);
    w.build(ui, || {

        ui.text(state.notify_text);

        // Set colour.
        for i in 0..5 {
            if ColorButton::new(ig_make_label!("Selector", i.to_string()), state.colors[i])
                .size([30.0,30.0])
                .tooltip(false)
                .build(ui){
                    state.current = state.colors[i];
                    state.notify_text = "Selector Clicked.";
                }
            ui.same_line_with_spacing(0.0, 5.0);
        }
        ui.new_line();
        ui.new_line();

        block_buttons(&ui, 3, 3, 0, state);
        ui.new_line();

        let row_width: f32 = 30.0 * 3.0 + 0.5 * 3.0;
        let padding: f32 = 20.0;
        for i in 0..4{
            block_buttons(&ui, 3, 3, i + 1, state);
            ui.same_line_with_spacing(ui.cursor_pos()[0], (row_width + padding) * (i + 1) as f32);
        }

        ui.new_line();
        ui.new_line();
        
        block_buttons(&ui, 3, 3, 5, state);
        ui.new_line();

        ui.text("This button changes colour when you click it");
        if ColorButton::new(im_str!("Changing Colour"), state.col).build(ui)
        {
            let dum: [f32; 4] = state.col;
            state.col = state.switch;
            state.switch = dum;
        }
    });
}

//#[derive(Default)]
struct State {
    col: [f32; 4],
    switch: [f32; 4],
    colors: [[f32; 4]; 6],
    current: [f32; 4],
    // Rubiks cube array.
    // 0
    // 1 2 3 4
    // 5
    rubiks: [[f32; 4]; 54],
    notify_text: &'static str,
}

impl State {
    fn reset(&mut self) {
        //self.notify_text = "";
    }
}

fn create_terminal(){
    // Command line
    let mut not_exit = true;
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
                    println!("Insert your U, D, L, R, F or B (each repressenting a different colour of the cube.)The first nine values should represent the Upper face, the next the right face, then the front face, down, left and finally back. Each face should describe the top left to bottom right facelets. ")
                } else if cube.len() > 54 {
                    println!("Your input has more facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else if cube.len() < 54 {
                    println!("Your input has less facelets than in a 3x3 rubiks cube at: {}. Please insure you have 54 facelets", cube.len())
                } else {
                    println!("Valid input; looking for moves now!");
                    let face = facelets::Face::new(&cube);
                    let return_code = face.check_if_can_be_solved();
                    println!("Return code is: {}", return_code);
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
    println!("Goodbye!");
}

fn main() {
    create_window();
    //create_terminal();
}
