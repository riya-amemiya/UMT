import timeit
import sys
import os

# We assume the script is run with PYTHONPATH set to package/umt_python (root of the project)
# or that we are running from package/umt_python
# We want to import src.math.subtraction

try:
    from src.math.subtraction import subtraction
except ImportError:
    # If running where umt_python is the package name (installed mode or configured)
    try:
        from umt_python.math.subtraction import subtraction  # type: ignore
    except ImportError:
        print(
            "Could not import subtraction. Make sure PYTHONPATH includes package/umt_python"
        )
        sys.exit(1)


def run_benchmark():
    print("Benchmarking subtraction...")
    number = 10000

    # Case 1: Simple decimal subtraction 0.3 - 0.1
    t1 = timeit.timeit(lambda: subtraction(0.3, 0.1), number=number)
    print(f"subtraction(0.3, 0.1) [{number} runs]: {t1:.6f}s")

    # Case 2: Integer subtraction 5 - 3
    t2 = timeit.timeit(lambda: subtraction(5, 3), number=number)
    print(f"subtraction(5, 3) [{number} runs]: {t2:.6f}s")

    # Case 3: Multiple arguments
    t3 = timeit.timeit(lambda: subtraction(1.0, 0.1, 0.2, 0.3, 0.1), number=number)
    print(f"subtraction(1.0, 0.1, 0.2, 0.3, 0.1) [{number} runs]: {t3:.6f}s")

    # Case 4: Larger numbers (more digits potentially affecting string ops)
    t4 = timeit.timeit(lambda: subtraction(12345.6789, 123.456, 12.34), number=number)
    print(f"subtraction(12345.6789, 123.456, 12.34) [{number} runs]: {t4:.6f}s")


if __name__ == "__main__":
    run_benchmark()
