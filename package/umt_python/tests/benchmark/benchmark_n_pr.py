import timeit
import math
import sys
import os

# Add package root to python path to allow imports like 'from src.math...'
package_root = os.path.dirname(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
)
sys.path.append(package_root)

from src.math.n_pr import n_pr


def benchmark():
    # Adjust path to reach package root from this file location
    current_dir = os.path.dirname(os.path.abspath(__file__))
    package_root = os.path.dirname(os.path.dirname(current_dir))

    setup = f"""
import sys
import os
package_root = r"{package_root}"
if package_root not in sys.path:
    sys.path.append(package_root)
from src.math.n_pr import n_pr
import math
"""

    # Test cases: (n, r)
    cases = [
        (10, 2),
        (100, 5),
        (1000, 10),
        (10000, 100),
        (20000, 1000),
    ]

    print(
        f"{'n':<10} {'r':<10} {'Current (us)':<15} {'math.perm (us)':<15} {'Speedup':<10}"
    )
    print("-" * 65)

    for n, r in cases:
        # Measure current implementation
        stmt_current = f"n_pr({n}, {r})"
        try:
            time_current = timeit.timeit(stmt_current, setup=setup, number=10000)
        except Exception as e:
            print(f"Error measuring n_pr: {e}")
            continue

        # Measure math.perm
        stmt_math = f"math.perm({n}, {r})"
        try:
            time_math = timeit.timeit(stmt_math, setup="import math", number=10000)
        except Exception as e:
            print(f"Error measuring math.perm: {e}")
            continue

        # Convert to microseconds per call
        us_current = (time_current / 10000) * 1e6
        us_math = (time_math / 10000) * 1e6
        speedup = us_current / us_math if us_math > 0 else 0

        print(f"{n:<10} {r:<10} {us_current:<15.4f} {us_math:<15.4f} {speedup:<10.2f}x")


if __name__ == "__main__":
    benchmark()
