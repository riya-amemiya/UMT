pub fn umt_factorize(n: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut n = n;

    if n == 0{
        return result;
    }
    else if n == 1{
        result.push(1);
        return result;
    }
    for i in 2..=n {
        while n % i == 0 {
            result.push(i);
            n /= i;
        }
    }
    result
}
