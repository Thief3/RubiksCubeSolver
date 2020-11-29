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
use crate::defs;
use crate::defs::facelets::Facelets as Facelets;
use crate::defs::facelets::Color as Color;
use crate::defs::facelets::GetChar;
use super::support as support;

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

/*
#[allow(dead_code)]
fn convert_color_rubiks_to_facelets(rubiks: [Color; 54]) -> facelets::Face{
    let mut a: facelets::Face = [facelets::Facelets::U; 54];
    // Remap the way they are in the gui to the old order required for the algo.
    // Upper
    for i in 0..9{
        // 0 - 8
        a[i] = rubiks[i].get_facelet();
    }
    // Left
    for i in 9..18{
        a[i + 3 * 9] = rubiks[i].get_facelet();
    }
    // Front
    for i in 18..27{
        a[i] = rubiks[i].get_facelet();
    }
    // Right
    for i in 27..36{
        a[i - 9 * 2] = rubiks[i].get_facelet();
    }
    // Back
    for i in 36..45{
        a[i + 9] = rubiks[i].get_facelet();
    }
    // Down
    for i in 45..54{
        a[i - 9 * 2] = rubiks[i].get_facelet();
    }

    return a;
}*/

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
    
    let system = support::init(file!());
    system.main_loop(move |_run, ui| {
        rubiks_cube_flat(ui, &mut state);
    });
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
        for i in 0..6 {
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
            //let r = convert_color_rubiks_to_chars(state.rubiks).iter().cloned().collect::<String>();
            //let face = facelets::Face::new(&r);
            //print!("{:?}", face);
            //let (a, b) = face.return_code_matcher();
            //state.notify_text = a;
            //if b {
               // let mut cube = face.turn_into_cube();
                //let moves = solver::complete_search(&mut cube);
                //let s = format!("Moves: {:?}", moves);
                
                // Memory Leak!!! Shouldn't be a problem, but it could be.
                //state.notify_text = Box::leak(s.into_boxed_str());
                //print!("{}", moves);
            //}
        }
    });
}
