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
//! Module that provides a GUI for the program. Written with imgui-rs, bindings
//! to Dear ImGui, an immediate mode gui library.


use imgui::*;

use crate::facelets;
use crate::solver;
use crate::ui_support;

pub struct State {
    colors: [Color; 6],
    current: Color,
    // Rubiks cube array.
    // 0
    // 1 2 3 4
    // 5
    rubiks: [Color; 54],
    notify_text: &'static str,
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
    #[allow(dead_code)]
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

#[allow(dead_code)]
fn convert_color_rubiks_to_facelets(rubiks: [Color; 54]) -> facelets::RubiksFacelets{
    let mut a: facelets::RubiksFacelets = [facelets::Facelets::U; 54];
    for i in 0..54{
        a[i] = rubiks[i].get_facelet();
    }
    return a;
}

pub fn create_window(){
    let mut state = State{
        colors: [
            Color::White,
            Color::Red,
            Color::Blue,
            Color::Orange,
            Color::Green,
            Color::Yellow,
        ],
        current: Color::White,
        rubiks: [Color::White; 54],
        notify_text: "",
    };

    for i in 0..6{
        for j in 0..9 {
            state.rubiks[i * 9 + j] = state.colors[i];
        }
    }
    
    let system = ui_support::init(file!());
    system.main_loop(move |_run, ui| {
        rubiks_cube_flat(ui, &mut state);
    });
}

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
                            state.rubiks[(row + x) as usize].get_vec())
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

pub fn rubiks_cube_flat(ui: &Ui, state: &mut State) {
    let w = Window::new(im_str!("Rubiks Cube Solver"))
        .size([600.0, 500.0], Condition::FirstUseEver)
        .position([20.0, 140.0], Condition::FirstUseEver);
    w.build(ui, || {

        // Start search
        
        ui.text(state.notify_text);

        // Set colour.
        for i in 0..5 {
            if ColorButton::new(ig_make_label!("Selector", i.to_string()), state.colors[i].get_vec())
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

        if ui.button(im_str!("Solve!"), [90.0, 30.0]) {
            let r = convert_color_rubiks_to_chars(state.rubiks).iter().cloned().collect::<String>();
            let face = facelets::Face::new(&r);
            let (a, b) = face.return_code_matcher();
            state.notify_text = a;
            if b {
                let mut cube = face.turn_into_cube();
                let moves = solver::complete_search(&mut cube);
                //state.notify_text = &moves;
                print!("{}", moves);
            }
        }
    });
}