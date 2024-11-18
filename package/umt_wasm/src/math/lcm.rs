use super::umt_gcd;

pub fn umt_lcm(a: i32, b: i32) -> i32 {
    a * b / umt_gcd(a, b)
}
