use umt_rust::math::{PrimeFactor, umt_prime_factorization};

#[test]
fn test_prime_factorization_single_prime() {
    let result = umt_prime_factorization(8);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 2,
            count: 3
        }]
    );
}

#[test]
fn test_prime_factorization_27() {
    let result = umt_prime_factorization(27);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 3,
            count: 3
        }]
    );
}

#[test]
fn test_prime_factorization_multiple_primes() {
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
fn test_prime_factorization_prime_number() {
    let result = umt_prime_factorization(2);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 2,
            count: 1
        }]
    );
}

#[test]
fn test_prime_factorization_17() {
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
fn test_prime_factorization_31() {
    let result = umt_prime_factorization(31);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 31,
            count: 1
        }]
    );
}

#[test]
fn test_prime_factorization_perfect_squares() {
    let result = umt_prime_factorization(25);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 5,
            count: 2
        }]
    );
}

#[test]
fn test_prime_factorization_49() {
    let result = umt_prime_factorization(49);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 7,
            count: 2
        }]
    );
}

#[test]
fn test_prime_factorization_large_prime() {
    let result = umt_prime_factorization(997);
    assert_eq!(
        result,
        vec![PrimeFactor {
            number: 997,
            count: 1
        }]
    );
}

#[test]
fn test_prime_factorization_998() {
    let result = umt_prime_factorization(998);
    assert_eq!(
        result,
        vec![
            PrimeFactor {
                number: 2,
                count: 1
            },
            PrimeFactor {
                number: 499,
                count: 1
            }
        ]
    );
}

#[test]
fn test_prime_factorization_consecutive() {
    let result = umt_prime_factorization(30);
    assert_eq!(
        result,
        vec![
            PrimeFactor {
                number: 2,
                count: 1
            },
            PrimeFactor {
                number: 3,
                count: 1
            },
            PrimeFactor {
                number: 5,
                count: 1
            }
        ]
    );
}

use umt_rust::math::*;

#[test]
fn test_prime_factorization_1() {
    let result = umt_prime_factorization(1);
    assert!(result.is_empty());
}

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
