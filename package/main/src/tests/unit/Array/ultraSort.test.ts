import { ultraSort } from "@/Array/ultraSort";

describe("ultraSort", () => {
  // Basic functionality tests
  it("should return an empty array when sorting an empty array", () => {
    expect(ultraSort([])).toEqual([]);
  });

  it("should return single-element array unchanged", () => {
    expect(ultraSort([1])).toEqual([1]);
  });

  // Network sort tests (arrays with 2-5 elements)
  describe("network sort for tiny arrays", () => {
    it("should sort 2-element arrays", () => {
      expect(ultraSort([2, 1])).toEqual([1, 2]);
      expect(ultraSort([1, 2])).toEqual([1, 2]);
    });

    it("should sort 3-element arrays in all permutations", () => {
      expect(ultraSort([1, 2, 3])).toEqual([1, 2, 3]);
      expect(ultraSort([1, 3, 2])).toEqual([1, 2, 3]);
      expect(ultraSort([2, 1, 3])).toEqual([1, 2, 3]);
      expect(ultraSort([2, 3, 1])).toEqual([1, 2, 3]);
      expect(ultraSort([3, 1, 2])).toEqual([1, 2, 3]);
      expect(ultraSort([3, 2, 1])).toEqual([1, 2, 3]);
    });

    it("should sort 4-element arrays", () => {
      expect(ultraSort([4, 3, 2, 1])).toEqual([1, 2, 3, 4]);
      expect(ultraSort([1, 4, 2, 3])).toEqual([1, 2, 3, 4]);
      expect(ultraSort([3, 1, 4, 2])).toEqual([1, 2, 3, 4]);
    });

    it("should sort 5-element arrays", () => {
      expect(ultraSort([5, 4, 3, 2, 1])).toEqual([1, 2, 3, 4, 5]);
      expect(ultraSort([3, 1, 4, 5, 2])).toEqual([1, 2, 3, 4, 5]);
      expect(ultraSort([2, 5, 1, 4, 3])).toEqual([1, 2, 3, 4, 5]);
    });
  });

  // Insertion sort threshold tests (6-24 elements)
  describe("insertion sort for small arrays", () => {
    it("should sort arrays at insertion sort threshold", () => {
      const arr = Array.from({ length: 24 }, (_, i) => 24 - i);
      expect(ultraSort(arr)).toEqual(
        Array.from({ length: 24 }, (_, i) => i + 1),
      );
    });

    it("should sort arrays below insertion sort threshold", () => {
      const arr = Array.from({ length: 15 }, (_, i) => 15 - i);
      expect(ultraSort(arr)).toEqual(
        Array.from({ length: 15 }, (_, i) => i + 1),
      );
    });
  });

  // Presortedness detection tests
  describe("presortedness detection", () => {
    it("should detect already sorted arrays", () => {
      const sorted = Array.from({ length: 100 }, (_, i) => i);
      const result = ultraSort([...sorted]);
      expect(result).toEqual(sorted);
    });

    it("should detect reverse sorted arrays", () => {
      const reversed = Array.from({ length: 100 }, (_, i) => 100 - i);
      const expected = Array.from({ length: 100 }, (_, i) => i + 1);
      expect(ultraSort(reversed)).toEqual(expected);
    });

    it("should handle partially sorted arrays", () => {
      const arr = [1, 2, 3, 4, 5, 9, 8, 7, 6, 10];
      expect(ultraSort(arr)).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    });
  });

  // Large array tests (introsort)
  describe("introsort for large arrays", () => {
    it("should sort large random arrays", () => {
      const size = 1000;
      const arr = Array.from({ length: size }, () =>
        Math.floor(Math.random() * size),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with many duplicates", () => {
      const arr = Array.from({ length: 1000 }, () =>
        Math.floor(Math.random() * 10),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays that trigger heapsort (worst case)", () => {
      // Create a pathological case that exhausts depth limit
      const size = 1000;
      const arr: number[] = [];
      for (let i = 0; i < size; i++) {
        arr.push(i % 2 === 0 ? i : size - i);
      }
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });
  });

  // Custom comparison function tests
  describe("custom comparison functions", () => {
    it("should sort in descending order", () => {
      const arr = [3, 1, 4, 1, 5, 9, 2, 6];
      expect(ultraSort(arr, (a, b) => b - a)).toEqual([9, 6, 5, 4, 3, 2, 1, 1]);
    });

    it("should sort strings", () => {
      const arr = ["banana", "apple", "cherry", "date"];
      expect(ultraSort(arr)).toEqual(["apple", "banana", "cherry", "date"]);
    });

    it("should sort objects by property", () => {
      const arr = [
        { name: "John", age: 30 },
        { name: "Jane", age: 25 },
        { name: "Bob", age: 35 },
      ];
      const sorted = ultraSort(arr, (a, b) => a.age - b.age);
      expect(sorted).toEqual([
        { name: "Jane", age: 25 },
        { name: "John", age: 30 },
        { name: "Bob", age: 35 },
      ]);
    });

    it("should handle comparison functions that return non-standard values", () => {
      const arr = [3, 1, 4, 1, 5];
      // Comparison function that returns large values
      const sorted = ultraSort(arr, (a, b) => (a - b) * 100);
      expect(sorted).toEqual([1, 1, 3, 4, 5]);
    });
  });

  // Edge cases
  describe("edge cases", () => {
    it("should handle arrays with negative numbers", () => {
      expect(ultraSort([5, -1, 3, -5, 0, 2])).toEqual([-5, -1, 0, 2, 3, 5]);
    });

    it("should handle arrays with floating point numbers", () => {
      expect(ultraSort([3.14, 2.71, 1.41, 1.73])).toEqual([
        1.41, 1.73, 2.71, 3.14,
      ]);
    });

    it("should handle arrays with all same values", () => {
      expect(ultraSort([5, 5, 5, 5, 5])).toEqual([5, 5, 5, 5, 5]);
    });

    it("should handle arrays with undefined values", () => {
      const arr = [3, undefined, 1, undefined, 2];
      const sorted = ultraSort(arr);
      expect(sorted.filter((x) => x !== undefined)).toEqual([1, 2, 3]);
    });

    it("should handle arrays with mixed types", () => {
      const arr = [3, "2", 1, "4"] as any[];
      const sorted = ultraSort(arr);
      expect(sorted).toEqual([1, "2", 3, "4"]);
    });

    it("should handle comparison function throwing errors", () => {
      const arr = [1, 2, 3];
      const errorCompare = () => {
        throw new Error("Comparison error");
      };
      expect(() => ultraSort(arr, errorCompare)).toThrow("Comparison error");
    });
  });

  // Pivot selection tests
  describe("pivot selection strategies", () => {
    it("should handle small arrays with median-of-three", () => {
      const arr = Array.from({ length: 50 }, () =>
        Math.floor(Math.random() * 100),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle large arrays with ninther", () => {
      const arr = Array.from({ length: 200 }, () =>
        Math.floor(Math.random() * 1000),
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with patterns that challenge pivot selection", () => {
      // Organ pipe pattern
      const arr1 = [
        ...Array.from({ length: 50 }, (_, i) => i),
        ...Array.from({ length: 50 }, (_, i) => 49 - i),
      ];
      const sorted1 = [...arr1].sort((a, b) => a - b);
      expect(ultraSort(arr1)).toEqual(sorted1);

      // Sawtooth pattern
      const arr2 = Array.from({ length: 100 }, (_, i) => i % 10);
      const sorted2 = [...arr2].sort((a, b) => a - b);
      expect(ultraSort(arr2)).toEqual(sorted2);
    });
  });

  // Three-way partitioning tests
  describe("three-way partitioning", () => {
    it("should efficiently handle arrays with many equal elements", () => {
      const arr: number[] = Array.from({ length: 1000 }, (_, i) => {
        if (i < 300) return 1;
        if (i < 600) return 2;
        if (i < 900) return 3;
        return 4;
      });
      // Shuffle the array
      for (let i = arr.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [arr[i], arr[j]] = [arr[j], arr[i]];
      }
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays where all elements equal the pivot", () => {
      const arr = Array.from({ length: 100 }, () => 42);
      expect(ultraSort(arr)).toEqual(arr);
    });
  });

  // Stability tests (ultraSort is not stable, but we can test consistency)
  describe("consistency tests", () => {
    it("should produce consistent results for arrays with equal elements", () => {
      const arr = [
        { id: 1, value: 5 },
        { id: 2, value: 3 },
        { id: 3, value: 5 },
        { id: 4, value: 3 },
        { id: 5, value: 5 },
      ];
      const sorted = ultraSort(arr, (a, b) => a.value - b.value);

      // Check that all elements with value 3 come before all elements with value 5
      const values = sorted.map((item) => item.value);
      expect(values).toEqual([3, 3, 5, 5, 5]);
    });
  });

  // Performance edge cases
  describe("performance edge cases", () => {
    it("should handle arrays with ascending runs", () => {
      const arr = [
        ...Array.from({ length: 100 }, (_, i) => i),
        ...Array.from({ length: 100 }, (_, i) => i + 200),
        ...Array.from({ length: 100 }, (_, i) => i + 100),
      ];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with descending runs", () => {
      const arr = [
        ...Array.from({ length: 100 }, (_, i) => 299 - i),
        ...Array.from({ length: 100 }, (_, i) => 199 - i),
        ...Array.from({ length: 100 }, (_, i) => 99 - i),
      ];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });

    it("should handle arrays with repeated patterns", () => {
      const pattern = [1, 5, 3, 8, 2, 7, 4, 6];
      const arr = Array.from(
        { length: 256 },
        (_, i) => pattern[i % pattern.length],
      );
      const sorted = [...arr].sort((a, b) => a - b);
      expect(ultraSort(arr)).toEqual(sorted);
    });
    // Specific branch coverage tests
    describe("branch coverage tests", () => {
      it("should handle checkPresortedness with exactly matching limits", () => {
        // Test array with exactly PARTIAL_INSERTION_SORT_LIMIT elements
        const arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraSort(arr)).toEqual(sorted);
      });

      it("should handle arrays where comparison returns 0", () => {
        const arr = [{ v: 1 }, { v: 1 }, { v: 2 }, { v: 2 }];
        const compareEqual = (a: any, b: any) => {
          if (a.v === b.v) return 0;
          return a.v - b.v;
        };
        const sorted = ultraSort(arr, compareEqual);
        expect(
          sorted.every((item, i) => i === 0 || sorted[i - 1].v <= item.v),
        ).toBe(true);
      });

      it("should handle medianOfThree with equal elements", () => {
        // Force medianOfThree to handle equal elements
        const arr = [5, 5, 5, 1, 2, 3, 5, 5, 5];
        expect(ultraSort(arr)).toEqual([1, 2, 3, 5, 5, 5, 5, 5, 5]);
      });

      it("should test all branches of choosePivot", () => {
        // Test array sizes that trigger different pivot strategies
        // Size < NINTHER_THRESHOLD (128)
        const small = Array.from({ length: 100 }, () => Math.random() * 100);
        const smallSorted = [...small].sort((a, b) => a - b);
        expect(ultraSort(small)).toEqual(smallSorted);

        // Size >= NINTHER_THRESHOLD
        const large = Array.from({ length: 200 }, () => Math.random() * 100);
        const largeSorted = [...large].sort((a, b) => a - b);
        expect(ultraSort(large)).toEqual(largeSorted);
      });

      it("should handle edge indices in choosePivot", () => {
        // Test with array size that makes gap calculations interesting
        const arr = Array.from({ length: 256 }, (_, i) => {
          // Create a pattern that tests edge cases in ninther
          if (i < 32) return i;
          if (i < 64) return 256 - i;
          if (i < 128) return i % 32;
          if (i < 192) return Math.floor(i / 2);
          return (i * 2) % 256;
        });
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraSort(arr)).toEqual(sorted);
      });

      it("should handle arrays where low + gap calculations need bounds checking", () => {
        // Small array that would cause out of bounds without Math.min checks
        const arr = Array.from({ length: 130 }, () => Math.random() * 100);
        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraSort(arr)).toEqual(sorted);
      });
    });

    // Additional heapsort trigger tests
    describe("heapsort trigger conditions", () => {
      it("should trigger heapsort with deeply nested similar values", () => {
        // Create an array that will exhaust the depth limit
        const size = 1000;
        const arr: number[] = [];

        // Create a pattern that causes bad pivot selection
        for (let i = 0; i < size / 2; i++) {
          arr.push(1);
        }
        for (let i = size / 2; i < size; i++) {
          arr.push(2);
        }

        // Add some noise in the middle
        for (let i = 0; i < 10; i++) {
          const idx = Math.floor(size / 2 - 5 + i);
          arr[idx] = Math.random() * 3;
        }

        const sorted = [...arr].sort((a, b) => a - b);
        expect(ultraSort(arr)).toEqual(sorted);
      });
      // reverseArray function coverage
      describe("reverseArray coverage", () => {
        it("should trigger reverseArray for reverse sorted arrays", () => {
          // Test with exact PARTIAL_INSERTION_SORT_LIMIT descending elements
          const arr = Array.from({ length: 8 }, (_, i) => 8 - i);
          expect(ultraSort(arr)).toEqual([1, 2, 3, 4, 5, 6, 7, 8]);
        });

        it("should handle checkPresortedness edge cases", () => {
          // Array with length < 2
          expect(ultraSort([1])).toEqual([1]);

          // Array with mixed ordering within limit
          const arr = [5, 4, 3, 4, 5, 6, 7, 8, 9, 10];
          const sorted = [...arr].sort((a, b) => a - b);
          expect(ultraSort(arr)).toEqual(sorted);
        });
      });

      // Final comprehensive test
      describe("comprehensive edge case coverage", () => {
        it("should handle all special cases comprehensively", () => {
          // Test type coercion in comparison
          const arr1 = [3, "2", 1, "4", 5] as any[];
          const result1 = ultraSort(arr1);
          expect(result1[0]).toBe(1);
          expect(result1[result1.length - 1]).toBe(5);

          // Test with objects that have valueOf
          const obj1 = { valueOf: () => 3 };
          const obj2 = { valueOf: () => 1 };
          const obj3 = { valueOf: () => 2 };
          const arr2 = [obj1, obj2, obj3] as any[];
          const sorted2 = ultraSort(arr2);
          expect(sorted2[0]).toBe(obj2);
          expect(sorted2[2]).toBe(obj1);
        });
      });
    });
  });
});
