/// A prime factor with its count.
#[derive(Debug, Clone, PartialEq)]
pub struct PrimeFactor {
    pub number: i64,
    pub count: i32,
}

/// Performs prime factorization of a number.
///
/// Returns an array of objects containing prime factors and their counts.
///
/// # Arguments
///
/// * `x` - Number to factorize.
///
/// # Returns
///
/// A vector of PrimeFactor structs containing prime factors and their counts.
///
/// # Examples
///
/// ```
/// use umt_rust::math::{umt_prime_factorization, PrimeFactor};
///
/// let result = umt_prime_factorization(12);
/// assert_eq!(result, vec![
///     PrimeFactor { number: 2, count: 2 },
///     PrimeFactor { number: 3, count: 1 }
/// ]);
/// // 12 = 2^2 * 3^1
/// ```
pub fn umt_prime_factorization(x: i64) -> Vec<PrimeFactor> {
    let mut copy_x = x;
    let mut out: Vec<PrimeFactor> = vec![];

    let mut index = 2i64;
    while index * index <= copy_x {
        if copy_x % index == 0 {
            let mut n = 0;
            while copy_x % index == 0 {
                n += 1;
                copy_x /= index;
            }
            out.push(PrimeFactor {
                number: index,
                count: n,
            });
        }
        index += 1;
    }

    // If remaining value is greater than 1, it's a prime factor
    if copy_x > 1 {
        out.push(PrimeFactor {
            number: copy_x,
            count: 1,
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factorization_12() {
        let result = umt_prime_factorization(12);
        assert_eq!(
            result,
            vec![
                PrimeFactor {
                    number: 2,
                    count: 2
                },
                PrimeFactor {
                    number: 3,
                    count: 1
                }
            ]
        );
    }

    #[test]
    fn test_prime_factorization_prime() {
        let result = umt_prime_factorization(17);
        assert_eq!(
            result,
            vec![PrimeFactor {
                number: 17,
                count: 1
            }]
        );
    }

    #[test]
    fn test_prime_factorization_power_of_2() {
        let result = umt_prime_factorization(16);
        assert_eq!(
            result,
            vec![PrimeFactor {
                number: 2,
                count: 4
            }]
        );
    }

    #[test]
    fn test_prime_factorization_100() {
        let result = umt_prime_factorization(100);
        assert_eq!(
            result,
            vec![
                PrimeFactor {
                    number: 2,
                    count: 2
                },
                PrimeFactor {
                    number: 5,
                    count: 2
                }
            ]
        );
    }

    #[test]
    fn test_prime_factorization_1() {
        let result = umt_prime_factorization(1);
        assert!(result.is_empty());
    }
}
