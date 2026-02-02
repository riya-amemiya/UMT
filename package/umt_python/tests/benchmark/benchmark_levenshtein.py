import time
import tracemalloc
import random
import string
import os
import sys

# Add package/umt_python to python path
current_dir = os.path.dirname(os.path.abspath(__file__))
package_root = os.path.abspath(os.path.join(current_dir, "../.."))
if package_root not in sys.path:
    sys.path.append(package_root)

from src.string.levenshtein_distance import levenshtein_distance


def generate_random_string(length):
    return "".join(random.choices(string.ascii_letters, k=length))


def run_benchmark():
    length = 2000  # Enough to see memory difference
    str1 = generate_random_string(length)
    str2 = generate_random_string(length)

    print(f"Benchmarking Levenshtein distance with string lengths {length}...")

    tracemalloc.start()
    start_time = time.time()

    dist = levenshtein_distance(str1, str2)

    end_time = time.time()
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    print(f"Result: {dist}")
    print(f"Time: {end_time - start_time:.4f} seconds")
    print(f"Peak Memory: {peak / 1024:.2f} KB")


if __name__ == "__main__":
    run_benchmark()
