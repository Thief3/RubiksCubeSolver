pub fn factorial(n : i64) -> i64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}