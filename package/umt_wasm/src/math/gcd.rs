pub fn umt_gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}
