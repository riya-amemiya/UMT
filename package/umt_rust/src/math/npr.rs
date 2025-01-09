use super::umt_factorial;

pub fn umt_npr(n: i32, r: i32) -> i32 {
    umt_factorial(n) / umt_factorial(n - r)
}
