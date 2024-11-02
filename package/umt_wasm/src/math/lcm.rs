use super::umt_gcd;

pub fn umt_lcm(a: i64, b: i64) -> i64 {
    a * b / umt_gcd(a, b)
}
