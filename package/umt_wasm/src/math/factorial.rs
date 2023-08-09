pub fn umt_factorial(mut x: i32) -> i32 {
    let mut result = 1;
    if x != 0 {
        while x > 1 {
            result *= x;
            x -= 1;
        }
    }
    return result;
}