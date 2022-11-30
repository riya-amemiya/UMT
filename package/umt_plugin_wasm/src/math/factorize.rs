pub fn umt_factorize(n: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut n = n;
    for i in 2..=n {
        while n % i == 0 {
            result.push(i);
            n /= i;
        }
    }
    result
}