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
//! A home for general purpose utilities. My own personal function list.

/// Calculates a factorial using recursive logic.
///
/// # Parameters
/// * `n` - Used to compute n!
/// # Outputs
/// * `i64` - The result of n!
pub fn factorial(n: i64) -> i64 {
    if n == 1 || n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Calculates a binomial coefficent.
///
/// # Parameters
/// * `n` - the number of objects
/// * `k` - the number of combinations
/// # Output
/// * `i64` - the binomial coefficent
pub fn binomial(n: i64, k: i64) -> i64 {
    if n > k {
    factorial(n) / (factorial(k) * factorial(n - k))
    }else{
        0
    }
}
