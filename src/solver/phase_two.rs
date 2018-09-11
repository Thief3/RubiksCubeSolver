//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

use physical;
use solver;

static PHASE_TWO_MOVE_LIST: [solver::Moves; 6] = [
    solver::Moves::U1,
    solver::Moves::B2,
    solver::Moves::F2,
    solver::Moves::D1,
    solver::Moves::L2,
    solver::Moves::R2,
];

fn phase_two_subgoal(rubiks: &mut physical::Cube) -> bool {
    (rubiks.corner_permutation == 0)
        && (rubiks.phase_two_edge_permutation == 0)
        && (rubiks.ud_sorted_slice == 0)
}
