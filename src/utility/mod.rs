//! A home for general purpose utilities. My own personal function list.

/// Calculates a factorial using recursive logic.
/// 
/// # Parameters
/// * `n` - Computes n!
/// # Outputs
/// * `i64` - The result of n!
pub fn factorial(n : i64) -> i64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}