import timeit
import sys
import os

# Ensure the script can import from src when run from the script's directory or the package root
script_dir = os.path.dirname(os.path.abspath(__file__))
package_root = os.path.abspath(os.path.join(script_dir, "../.."))
if package_root not in sys.path:
    sys.path.append(package_root)

try:
    from src.math.average import average
except ImportError:
    print(f"Could not import average. PYTHONPATH: {sys.path}")
    sys.exit(1)


def run_benchmark():
    print("Benchmarking average...")
    iterations = 1000

    # Case 1: Small list
    numbers_small = [0.1, 0.2, 0.3, 0.4, 0.5]
    t1 = timeit.timeit(lambda: average(numbers_small), number=iterations)
    print(f"average(small_list) [{iterations} runs]: {t1:.6f}s")

    # Case 2: Medium list
    numbers_medium = [float(i) / 10.0 for i in range(100)]
    t2 = timeit.timeit(lambda: average(numbers_medium), number=iterations)
    print(f"average(medium_list) [{iterations} runs]: {t2:.6f}s")

    # Case 3: Large list
    numbers_large = [float(i) / 10.0 for i in range(1000)]
    t3 = timeit.timeit(lambda: average(numbers_large), number=100) # Fewer iterations for large list
    print(f"average(large_list) [100 runs]: {t3:.6f}s")


if __name__ == "__main__":
    run_benchmark()
