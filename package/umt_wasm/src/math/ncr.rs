use super::{umt_npr, umt_factorial};

pub fn umt_ncr(n: i32, r: i32) -> i32 {
    umt_npr(n, r) / umt_factorial(r)
}