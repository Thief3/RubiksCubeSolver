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
    factorial(n) / (factorial(k) * factorial(n - k))
}