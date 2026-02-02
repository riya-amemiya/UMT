import timeit
import random
import sys
import os

# Add package root to path to allow imports
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../")))

from src.array.ultra_number_sort import ultra_number_sort  # noqa: E402


def run_benchmark() -> None:
    """Run benchmark for ultra_number_sort."""
    # Setup data
    random.seed(42)
    size = 10000
    iterations = 50

    # 1. Large Integers
    integers = [random.randint(0, 1000000) for _ in range(size)]

    # 2. Floats
    floats = [random.random() * 1000000 for _ in range(size)]

    # 3. Small Integers
    small_ints = [random.randint(0, 100) for _ in range(size)]

    # 4. NaNs
    nans = [random.random() for _ in range(size)]
    for i in range(0, size, 10):
        nans[i] = float("nan")

    print(f"Benchmarking with array size {size}, {iterations} iterations")

    # Benchmark 1: Integers
    def bench_int() -> None:
        ultra_number_sort(integers)

    t_int = timeit.timeit(bench_int, number=iterations)
    print(f"Integers:   {t_int:.4f}s")

    # Benchmark 2: Floats
    def bench_float() -> None:
        ultra_number_sort(floats)

    t_float = timeit.timeit(bench_float, number=iterations)
    print(f"Floats:     {t_float:.4f}s")

    # Benchmark 3: Counting
    def bench_cnt() -> None:
        ultra_number_sort(small_ints)

    t_cnt = timeit.timeit(bench_cnt, number=iterations)
    print(f"Small Ints: {t_cnt:.4f}s")

    # Benchmark 4: NaNs
    def bench_nan() -> None:
        ultra_number_sort(nans)

    t_nan = timeit.timeit(bench_nan, number=iterations)
    print(f"With NaNs:  {t_nan:.4f}s")

    # Comparison with native sort (for integers)
    def bench_native() -> None:
        sorted(integers)

    t_native = timeit.timeit(bench_native, number=iterations)
    print(f"Native:     {t_native:.4f}s")


if __name__ == "__main__":
    run_benchmark()
