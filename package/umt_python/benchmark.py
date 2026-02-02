import os
import sys
import timeit

# Ensure src can be imported
sys.path.append(os.path.join(os.path.dirname(__file__), "src"))

from src.math import prime_factorization


def benchmark() -> None:
    # Test cases:
    # 1. Large prime (worst case for loop up to sqrt(n))
    # 2. Large composite with small factors
    # 3. Large composite with large factors

    # 2147483647 is a prime (2^31 - 1). sqrt is ~46340.
    # This is small enough to be very fast.

    # Let's try something bigger.
    # 1000000007 is a prime.

    # We want something that takes e.g. 0.1s per run so 10 runs take 1s.
    # 10^14. sqrt is 10^7 = 10,000,000 iterations.
    # 9999999967 is the largest prime under 10^10. sqrt is 10^5.

    # Let's try a number around 10^14 which is prime or near prime.
    # 99999999999973 is prime. sqrt is ~10^7.
    target = 99999999999973

    def run() -> None:
        prime_factorization(target)

    # Run it 5 times
    number_of_runs = 5
    elapsed = timeit.timeit(run, number=number_of_runs)
    print(f"Time taken for {number_of_runs} runs on {target}: {elapsed:.4f} seconds")
    print(f"Average time: {elapsed / number_of_runs:.4f} seconds")


if __name__ == "__main__":
    benchmark()
