import { ultraNumberSort } from "@/Array/ultraNumberSort";

describe("ultraNumberSort", () => {
  // Basic functionality tests
  it("should return an empty array when sorting an empty array", () => {
    expect(ultraNumberSort([])).toEqual([]);
  });

  it("should return single-element array unchanged", () => {
    expect(ultraNumberSort([42])).toEqual([42]);
    expect(ultraNumberSort([42], false)).toEqual([42]);
  });

  // 2-element arrays
  describe("2-element arrays", () => {
    it("should sort in ascending order by default", () => {
      expect(ultraNumberSort([2, 1])).toEqual([1, 2]);
      expect(ultraNumberSort([1, 2])).toEqual([1, 2]);
    });

    it("should sort in descending order when specified", () => {
      expect(ultraNumberSort([1, 2], false)).toEqual([2, 1]);
      expect(ultraNumberSort([2, 1], false)).toEqual([2, 1]);
    });
  });

  // 3-element arrays (inline sort)
  describe("3-element arrays (inline sort)", () => {
    it("should sort all permutations in ascending order", () => {
      expect(ultraNumberSort([1, 2, 3])).toEqual([1, 2, 3]);
      expect(ultraNumberSort([1, 3, 2])).toEqual([1, 2, 3]);
      expect(ultraNumberSort([2, 1, 3])).toEqual([1, 2, 3]);
      expect(ultraNumberSort([2, 3, 1])).toEqual([1, 2, 3]);
      expect(ultraNumberSort([3, 1, 2])).toEqual([1, 2, 3]);
      expect(ultraNumberSort([3, 2, 1])).toEqual([1, 2, 3]);
    });

    it("should sort all permutations in descending order", () => {
      expect(ultraNumberSort([1, 2, 3], false)).toEqual([3, 2, 1]);
      expect(ultraNumberSort([1, 3, 2], false)).toEqual([3, 2, 1]);
      expect(ultraNumberSort([2, 1, 3], false)).toEqual([3, 2, 1]);
      expect(ultraNumberSort([2, 3, 1], false)).toEqual([3, 2, 1]);
      expect(ultraNumberSort([3, 1, 2], false)).toEqual([3, 2, 1]);
      expect(ultraNumberSort([3, 2, 1], false)).toEqual([3, 2, 1]);
    });
  });

  // NaN handling
  describe("NaN handling", () => {
    it("should move NaN values to the end", () => {
      expect(ultraNumberSort([3, Number.NaN, 1, Number.NaN, 2])).toEqual([
        1,
        2,
        3,
        Number.NaN,
        Number.NaN,
      ]);
      expect(ultraNumberSort([Number.NaN, Number.NaN, Number.NaN])).toEqual([
        Number.NaN,
        Number.NaN,
        Number.NaN,
      ]);
    });

    it("should handle NaN with descending order", () => {
      expect(ultraNumberSort([3, Number.NaN, 1, Number.NaN, 2], false)).toEqual(
        [3, 2, 1, Number.NaN, Number.NaN],
      );
    });

    it("should handle array with only NaN values", () => {
      const arr = [Number.NaN, Number.NaN, Number.NaN, Number.NaN];
      expect(ultraNumberSort(arr)).toEqual([
        Number.NaN,
        Number.NaN,
        Number.NaN,
        Number.NaN,
      ]);
    });

    it("should handle mixed NaN and valid numbers", () => {
      const arr = [Number.NaN, 5, Number.NaN, 3, 1, Number.NaN, 4, 2];
      expect(ultraNumberSort(arr)).toEqual([
        1,
        2,
        3,
        4,
        5,
        Number.NaN,
        Number.NaN,
        Number.NaN,
      ]);
    });
  });

  // Infinity handling
  describe("Infinity handling", () => {
    it("should handle positive and negative infinity", () => {
      expect(
        ultraNumberSort([
          Number.POSITIVE_INFINITY,
          3,
          Number.NEGATIVE_INFINITY,
          1,
          2,
        ]),
      ).toEqual([Number.NEGATIVE_INFINITY, 1, 2, 3, Number.POSITIVE_INFINITY]);
      expect(
        ultraNumberSort([Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]),
      ).toEqual([Number.NEGATIVE_INFINITY, Number.POSITIVE_INFINITY]);
    });

    it("should handle infinity in descending order", () => {
      expect(
        ultraNumberSort(
          [Number.POSITIVE_INFINITY, 3, Number.NEGATIVE_INFINITY, 1, 2],
          false,
        ),
      ).toEqual([Number.POSITIVE_INFINITY, 3, 2, 1, Number.NEGATIVE_INFINITY]);
    });

    it("should handle multiple infinities", () => {
      expect(
        ultraNumberSort([
          Number.POSITIVE_INFINITY,
          Number.POSITIVE_INFINITY,
          Number.NEGATIVE_INFINITY,
          Number.NEGATIVE_INFINITY,
          0,
        ]),
      ).toEqual([
        Number.NEGATIVE_INFINITY,
        Number.NEGATIVE_INFINITY,
        0,
        Number.POSITIVE_INFINITY,
        Number.POSITIVE_INFINITY,
      ]);
    });
  });

  // Integer counting sort
  describe("counting sort for small integer ranges", () => {
    it("should use counting sort for small ranges", () => {
      const arr = Array.from({ length: 100 }, () =>
        Math.floor(Math.random() * 50),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle negative integers in small ranges", () => {
      const arr = Array.from(
        { length: 100 },
        () => Math.floor(Math.random() * 100) - 50,
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with duplicate integers", () => {
      const arr = Array.from({ length: 1000 }, () =>
        Math.floor(Math.random() * 10),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });
  });

  // Radix sort for large integers
  describe("radix sort for large integer arrays", () => {
    it("should use radix sort for large positive integers", () => {
      const arr = Array.from({ length: 1000 }, () =>
        Math.floor(Math.random() * 1000000),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle mixed positive and negative integers", () => {
      const arr = Array.from(
        { length: 1000 },
        () => Math.floor(Math.random() * 2000000) - 1000000,
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with zeros", () => {
      const arr = [100, 0, -100, 0, 50, -50, 0];
      expect(ultraNumberSort(arr)).toEqual([-100, -50, 0, 0, 0, 50, 100]);
    });

    it("should handle large integers in descending order", () => {
      const arr = Array.from({ length: 500 }, () =>
        Math.floor(Math.random() * 100000),
      );
      const sorted = [...arr].sort((a, b) => b - a);
      expect(ultraNumberSort(arr, false)).toEqual(sorted);
    });
  });

  // Floating point numbers
  describe("floating point numbers", () => {
    it("should sort floating point numbers", () => {
      const arr = [3.14, 2.71, 1.41, 1.73, 2.23];
      expect(ultraNumberSort(arr)).toEqual([1.41, 1.73, 2.23, 2.71, 3.14]);
    });

    it("should handle very small floating point numbers", () => {
      const arr = [0.0001, 0.00001, 0.001, 0.000001];
      expect(ultraNumberSort(arr)).toEqual([0.000001, 0.00001, 0.0001, 0.001]);
    });

    it("should handle mixed integers and floats", () => {
      const arr = [3, 2.5, 1, 1.5, 2];
      expect(ultraNumberSort(arr)).toEqual([1, 1.5, 2, 2.5, 3]);
    });

    it("should handle negative floating point numbers", () => {
      const arr = [-3.14, -2.71, -1.41, -1.73];
      expect(ultraNumberSort(arr)).toEqual([-3.14, -2.71, -1.73, -1.41]);
    });
  });

  // Edge cases
  describe("edge cases", () => {
    it("should handle arrays with all same values", () => {
      const arr = Array(100).fill(42);
      expect(ultraNumberSort(arr)).toEqual(arr);
    });

    it("should handle arrays with -0 and +0", () => {
      const arr = [-0, 0, -0, 0];
      const sorted = ultraNumberSort(arr);
      expect(sorted.every((x) => x === 0)).toBe(true);
    });

    it("should handle very large arrays efficiently", () => {
      const arr = Array.from({ length: 10000 }, () => Math.random() * 1000);
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with extreme values", () => {
      const arr = [Number.MAX_VALUE, Number.MIN_VALUE, -Number.MAX_VALUE, 0];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with Number.EPSILON", () => {
      const arr = [1, 1 + Number.EPSILON, 1 - Number.EPSILON];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });
  });

  // Performance patterns
  describe("performance patterns", () => {
    it("should efficiently handle already sorted arrays", () => {
      const arr = Array.from({ length: 1000 }, (_, i) => i);
      expect(ultraNumberSort([...arr])).toEqual(arr);
    });

    it("should efficiently handle reverse sorted arrays", () => {
      const arr = Array.from({ length: 1000 }, (_, i) => 1000 - i);
      const expected = Array.from({ length: 1000 }, (_, i) => i + 1);
      expect(ultraNumberSort(arr)).toEqual(expected);
    });

    it("should handle arrays with many duplicates efficiently", () => {
      const arr = Array.from({ length: 10000 }, () =>
        Math.floor(Math.random() * 10),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle sawtooth patterns", () => {
      const arr = Array.from({ length: 1000 }, (_, i) => i % 100);
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle organ pipe patterns", () => {
      const arr = [
        ...Array.from({ length: 500 }, (_, i) => i),
        ...Array.from({ length: 500 }, (_, i) => 499 - i),
      ];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });
  });

  // Quicksort fallback tests
  describe("quicksort fallback for complex cases", () => {
    it("should handle arrays that don't fit counting or radix sort criteria", () => {
      // Large range of floating point numbers
      const arr = Array.from({ length: 1000 }, () => Math.random() * 1000000);
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with large integer ranges", () => {
      // Range too large for counting sort
      const arr = Array.from({ length: 100 }, () =>
        Math.floor(Math.random() * Number.MAX_SAFE_INTEGER),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });
  });

  // Insertion sort for small subarrays
  describe("insertion sort optimization", () => {
    it("should use insertion sort for small subarrays", () => {
      const arr = Array.from({ length: 15 }, () => Math.random() * 100);
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraNumberSort(arr)).toEqual(sorted);
    });
  });

  // Comprehensive random tests
  describe("comprehensive random tests", () => {
    it("should correctly sort various random arrays", () => {
      for (let i = 0; i < 10; i++) {
        const size = Math.floor(Math.random() * 1000) + 10;
        const arr = Array.from({ length: size }, () => {
          const type = Math.random();
          if (type < 0.1) return Number.NaN;
          if (type < 0.15) return Number.POSITIVE_INFINITY;
          if (type < 0.2) return Number.NEGATIVE_INFINITY;
          if (type < 0.5) return Math.floor(Math.random() * 100);
          if (type < 0.8) return Math.floor(Math.random() * 10000) - 5000;
          return Math.random() * 1000 - 500;
        });

        const expected = [...arr].sort((a, b) => {
          if (Number.isNaN(a) && Number.isNaN(b)) return 0;
          if (Number.isNaN(a)) return 1;
          if (Number.isNaN(b)) return -1;
          return a - b;
        });

        expect(ultraNumberSort(arr)).toEqual(expected);
      }
    });
    // Branch coverage tests
    describe("branch coverage tests", () => {
      it("should cover all branches in inlineSort3", () => {
        // Test all possible orderings for 3 elements
        const testCases = [
          [1, 2, 3], // a <= b <= c
          [1, 3, 2], // a <= c < b
          [2, 1, 3], // b < a <= c
          [2, 3, 1], // c < a < b
          [3, 1, 2], // b < c < a
          [3, 2, 1], // c < b < a
          [1, 1, 2], // a = b < c
          [1, 2, 1], // a = c < b
          [2, 1, 1], // b = c < a
          [1, 1, 1], // a = b = c
        ];

        testCases.forEach((arr) => {
          const ascExpected = [...arr].sort((a, b) => a - b);
          const descExpected = [...arr].sort((a, b) => b - a);

          expect(ultraNumberSort([...arr], true)).toEqual(ascExpected);
          expect(ultraNumberSort([...arr], false)).toEqual(descExpected);
        });
      });

      it("should handle counting sort boundary conditions", () => {
        // Test exact boundary for counting sort range
        const arr1 = Array.from({ length: 100 }, (_, i) => i * 2); // range = 198
        const sorted1 = [...arr1].sort((a, b) => a - b);
        expect(ultraNumberSort(arr1)).toEqual(sorted1);

        // Test just over the boundary
        const arr2 = Array.from({ length: 100 }, (_, i) => i * 20); // range = 1980
        const sorted2 = [...arr2].sort((a, b) => a - b);
        expect(ultraNumberSort(arr2)).toEqual(sorted2);
      });

      it("should handle radix sort with various bit patterns", () => {
        // Test numbers that exercise different bit shifts
        const arr = [
          0xff, // 255 - 8 bits
          0xffff, // 65535 - 16 bits
          0xffffff, // 16777215 - 24 bits
          0x7fffffff, // 2147483647 - 31 bits
          1,
          256,
          65536,
          16777216,
        ];
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle numericPartition median selection branches", () => {
        // Test arrays that trigger different branches in median-of-three
        const testCases = [
          [3, 1, 2, 4, 5], // mid < low case
          [1, 3, 2, 4, 5], // high < low case
          [2, 1, 3, 4, 5], // high < mid case
          [1, 2, 3, 4, 5], // already sorted
          [5, 4, 3, 2, 1], // reverse sorted
        ];

        testCases.forEach((arr) => {
          const sorted = [...arr].sort((a, b) => a - b);
          expect(ultraNumberSort([...arr])).toEqual(sorted);
        });

        // Test descending partition to cover line 390
        const descTestCases = [
          [1, 3, 2, 4.5, 5.5], // high > low case in descending
          [3, 1, 2, 4.5, 5.5], // mid > low case in descending
          [2, 1, 3, 4.5, 5.5], // high > mid case in descending
        ];

        descTestCases.forEach((arr) => {
          const sorted = [...arr].sort((a, b) => b - a);
          expect(ultraNumberSort([...arr], false)).toEqual(sorted);
        });
      });

      it("should handle quicksort stack operations", () => {
        // Test array that will exercise stack push order logic
        const arr = Array.from({ length: 100 }, (_, i) => {
          // Create unbalanced partitions
          if (i < 20) return i;
          if (i < 80) return 50;
          return 100 - i;
        });
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle descending order in all sort methods", () => {
        // Test descending with counting sort range
        const arr1 = Array.from({ length: 50 }, () =>
          Math.floor(Math.random() * 100),
        );
        const sorted1 = [...arr1].sort((a, b) => b - a);
        expect(ultraNumberSort(arr1, false)).toEqual(sorted1);

        // Test descending with radix sort
        const arr2 = Array.from({ length: 200 }, () =>
          Math.floor(Math.random() * 10000),
        );
        const sorted2 = [...arr2].sort((a, b) => b - a);
        expect(ultraNumberSort(arr2, false)).toEqual(sorted2);

        // Test descending with quicksort
        const arr3 = Array.from({ length: 100 }, () => Math.random() * 1000);
        const sorted3 = [...arr3].sort((a, b) => b - a);
        expect(ultraNumberSort(arr3, false)).toEqual(sorted3);
      });

      it("should handle arrays where allIntegers check fails", () => {
        // Start with integers then add a float
        const arr = [1, 2, 3, 4, 5, 3.5, 6, 7, 8, 9];
        expect(ultraNumberSort(arr)).toEqual([1, 2, 3, 3.5, 4, 5, 6, 7, 8, 9]);
      });

      it("should handle radixSort with only positive numbers", () => {
        const arr = Array.from(
          { length: 100 },
          () => Math.floor(Math.random() * 1000) + 1,
        );
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle radixSort with only negative numbers", () => {
        const arr = Array.from(
          { length: 100 },
          () => -Math.floor(Math.random() * 1000) - 1,
        );
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle numericInsertionSort branches", () => {
        // Test insertion sort with both ascending and descending
        const arr = [5, 2, 8, 1, 9, 3, 7, 4, 6];

        // This will use insertion sort due to small size
        expect(ultraNumberSort([...arr], true)).toEqual([
          1, 2, 3, 4, 5, 6, 7, 8, 9,
        ]);
        expect(ultraNumberSort([...arr], false)).toEqual([
          9, 8, 7, 6, 5, 4, 3, 2, 1,
        ]);
      });

      it("should handle numericPartition while loops", () => {
        // Create array that exercises partition while loops
        const arr = Array.from({ length: 50 }, (_, i) => {
          if (i % 3 === 0) return 10;
          if (i % 3 === 1) return 5;
          return 15;
        });

        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle edge case where max === min", () => {
        // All elements are the same (for counting sort branch)
        const arr = Array(100).fill(42);
        expect(ultraNumberSort(arr)).toEqual(arr);
      });

      it("should handle counting sort with exactly at threshold", () => {
        // Test where max - min === len * 2
        const arr = Array.from({ length: 50 }, (_, i) => i * 2);
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should test radixSortPositive with maximum bit shifts", () => {
        // Test with numbers that require maximum iterations
        const arr = [
          0x7fffffff, // Maximum positive 32-bit integer
          0x3fffffff,
          0x1fffffff,
          0x0fffffff,
          1,
          2,
          3,
        ];
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });
    });

    // Additional edge cases for 100% coverage
    describe("additional coverage tests", () => {
      it("should handle empty positive and negative arrays in radixSort", () => {
        // Only zeros
        const arr1 = [0, 0, 0, 0, 0];
        expect(ultraNumberSort(arr1)).toEqual([0, 0, 0, 0, 0]);

        // Mix that results in empty positive/negative arrays
        const arr2 = [0, -1, 0, 1, 0];
        expect(ultraNumberSort(arr2)).toEqual([-1, 0, 0, 0, 1]);
      });

      it("should handle stack edge cases in quicksort", () => {
        // Very small array for quicksort
        const arr = Array.from({ length: 17 }, () => Math.random());
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });

      it("should handle h === undefined || l === undefined case", () => {
        // This is defensive programming, but let's test the branch
        // Create array that might trigger edge case in partition
        const arr: number[] = Array.from({ length: 100 }, (_, i) => {
          // Pattern that creates specific partition sizes
          if (i < 25) return 1;
          if (i < 50) return 2;
          if (i < 75) return 3;
          return 4;
        });

        // Shuffle
        for (let i = arr.length - 1; i > 0; i--) {
          const j = Math.floor(Math.random() * (i + 1));
          [arr[i], arr[j]] = [arr[j], arr[i]];
        }

        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraNumberSort(arr)).toEqual(sorted);
      });
      // Final coverage for specific branches
      describe("final branch coverage", () => {
        it("should handle countingSort descending loop", () => {
          // Small range integers in descending order
          const arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
          expect(ultraNumberSort(arr, false)).toEqual([
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
          ]);
        });

        it("should handle radixSort with edge patterns", () => {
          // Only positive numbers with zeros
          const arr1 = [5, 0, 3, 0, 1, 0, 4, 2];
          expect(ultraNumberSort(arr1)).toEqual([0, 0, 0, 1, 2, 3, 4, 5]);
          expect(ultraNumberSort(arr1, false)).toEqual([
            5, 4, 3, 2, 1, 0, 0, 0,
          ]);

          // Only negative numbers
          const arr2 = [-5, -3, -1, -4, -2];
          expect(ultraNumberSort(arr2)).toEqual([-5, -4, -3, -2, -1]);
          expect(ultraNumberSort(arr2, false)).toEqual([-1, -2, -3, -4, -5]);

          // Edge case with single negative
          const arr3 = [-1];
          expect(ultraNumberSort(arr3)).toEqual([-1]);

          // Test descending radix sort with zeros (covers line 230)
          const arr4 = Array.from({ length: 150 }, (_, i) => {
            if (i < 50) return 0;
            if (i < 100) return i - 49;
            return -(i - 99);
          });
          const sorted4 = [...arr4].sort((a, b) => b - a);
          expect(ultraNumberSort(arr4, false)).toEqual(sorted4);

          // Test descending radix sort with negatives (covers line 234)
          const arr5 = Array.from({ length: 120 }, (_, i) => {
            if (i < 40) return i + 1;
            if (i < 80) return 0;
            return -(i - 79);
          });
          const sorted5 = [...arr5].sort((a, b) => b - a);
          expect(ultraNumberSort(arr5, false)).toEqual(sorted5);

          // Test radixSortPositive with single element array (covers line 247)
          const singleElementArray = Array.from({ length: 101 }, (_, i) =>
            i === 0 ? 1 : 0,
          );
          expect(ultraNumberSort(singleElementArray)).toEqual([
            ...Array(100).fill(0),
            1,
          ]);
        });

        it("should cover remaining uncovered lines", () => {
          const onlyZerosDesc = Array.from({ length: 150 }, () => 0);
          expect(ultraNumberSort(onlyZerosDesc, false)).toEqual(onlyZerosDesc);

          const onlyNegativesDesc = Array.from(
            { length: 120 },
            (_, i) => -(i + 1),
          );
          const expectedNegDesc = [...onlyNegativesDesc].sort((a, b) => b - a);
          expect(ultraNumberSort(onlyNegativesDesc, false)).toEqual(
            expectedNegDesc,
          );

          const singlePositiveArray = [42];
          expect(ultraNumberSort(singlePositiveArray)).toEqual([42]);
        });

        it("should handle counting sort with min > 0", () => {
          // All positive integers in small range
          const arr = [10, 12, 11, 13, 15, 14];
          expect(ultraNumberSort(arr)).toEqual([10, 11, 12, 13, 14, 15]);
          expect(ultraNumberSort(arr, false)).toEqual([15, 14, 13, 12, 11, 10]);
        });

        it("should handle counting sort with negative min", () => {
          // Negative integers in small range
          const arr = [-10, -8, -9, -7, -6];
          expect(ultraNumberSort(arr)).toEqual([-10, -9, -8, -7, -6]);
          expect(ultraNumberSort(arr, false)).toEqual([-6, -7, -8, -9, -10]);
        });

        it("should handle exact threshold conditions", () => {
          // Array length exactly 100 for radix sort threshold
          const arr = Array.from({ length: 100 }, (_, i) => i);
          const shuffled = [...arr];
          for (let i = shuffled.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
          }
          expect(ultraNumberSort(shuffled)).toEqual(arr);
        });

        it("should handle radixSortPositive with zero max", () => {
          // Edge case where all numbers are 0
          const arr = [0, 0, 0];
          expect(ultraNumberSort(arr)).toEqual([0, 0, 0]);
        });

        it("should handle numeric partition edge cases", () => {
          // Array that triggers specific partition paths
          const arr = [5, 5, 5, 3, 3, 3, 7, 7, 7];
          expect(ultraNumberSort(arr)).toEqual([3, 3, 3, 5, 5, 5, 7, 7, 7]);
        });

        it("should handle very specific counting sort range", () => {
          // Test max - min < len * 2 && max - min < 1000000
          const arr = Array.from({ length: 500001 }, (_, i) => i % 1000);
          const sorted = [...arr].sort((a, b) => a - b);
          expect(ultraNumberSort(arr)).toEqual(sorted);
        });

        it("should handle Float64Array operations in radix sort", () => {
          // Test with numbers that exercise Float64Array conversion
          const arr = Array.from({ length: 256 }, (_, i) => i * 256);
          const sorted = [...arr].sort((a, b) => a - b);
          expect(ultraNumberSort(arr)).toEqual(sorted);
        });

        it("should handle all zero count array in counting sort", () => {
          // Edge case where count array might be all zeros (shouldn't happen in practice)
          const arr = [5, 5, 5, 5, 5];
          expect(ultraNumberSort(arr)).toEqual([5, 5, 5, 5, 5]);
        });
      });

      // Ultimate edge case tests
      describe("ultimate edge cases", () => {
        it("should handle maximum safe integer", () => {
          const arr = [Number.MAX_SAFE_INTEGER, 0, -Number.MAX_SAFE_INTEGER];
          expect(ultraNumberSort(arr)).toEqual([
            -Number.MAX_SAFE_INTEGER,
            0,
            Number.MAX_SAFE_INTEGER,
          ]);
        });

        it("should handle subnormal numbers", () => {
          const arr = [Number.MIN_VALUE, -Number.MIN_VALUE, 0, 1e-300];
          const sorted = [...arr].sort((a, b) => a - b);
          expect(ultraNumberSort(arr)).toEqual(sorted);
        });

        it("should handle mixed special values", () => {
          const arr = [
            0,
            -0,
            Number.POSITIVE_INFINITY,
            Number.NEGATIVE_INFINITY,
            Number.NaN,
            1,
            -1,
          ];
          const sorted = ultraNumberSort(arr);

          expect(sorted[0]).toBe(Number.NEGATIVE_INFINITY);
          expect(sorted[sorted.length - 2]).toBe(Number.POSITIVE_INFINITY);
          expect(Number.isNaN(sorted[sorted.length - 1])).toBe(true);
        });
      });
    });
  });
});
