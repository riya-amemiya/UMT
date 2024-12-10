pub fn umt_gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}