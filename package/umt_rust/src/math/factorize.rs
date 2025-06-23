/// Factorizes a number into its prime factors.
///
/// This function takes an integer as input and returns a vector containing its prime factors.
///
/// # Arguments
///
/// * `n` - The integer to factorize.
///
/// # Returns
///
/// A vector containing the prime factors of the input integer.
pub fn umt_factorize(n: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut n = n;

    if n <= 1 {
        if n == 1 {
            result.push(1);
        }
        return result;
    }

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            result.push(i);
            n /= i;
        }
        i += 1;
    }

    if n > 1 {
        result.push(n);
    }

    result
}
