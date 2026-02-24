import { percentile } from "@/Math/percentile";

describe("percentile", () => {
  // ============================================================
  // 1. JSDoc examples
  // ============================================================
  describe("JSDoc examples", () => {
    it("should return 3 for 50th percentile of [1,2,3,4,5]", () => {
      expect(percentile([1, 2, 3, 4, 5], 50)).toBe(3);
    });

    it("should return 2 for 25th percentile of [1,2,3,4,5]", () => {
      expect(percentile([1, 2, 3, 4, 5], 25)).toBe(2);
    });

    it("should return 4 for 75th percentile of [1,2,3,4,5]", () => {
      expect(percentile([1, 2, 3, 4, 5], 75)).toBe(4);
    });
  });

  // ============================================================
  // 2. Empty array
  // ============================================================
  describe("empty array", () => {
    it("should return NaN for 0th percentile of empty array", () => {
      expect(percentile([], 0)).toBeNaN();
    });

    it("should return NaN for 50th percentile of empty array", () => {
      expect(percentile([], 50)).toBeNaN();
    });

    it("should return NaN for 100th percentile of empty array", () => {
      expect(percentile([], 100)).toBeNaN();
    });
  });

  // ============================================================
  // 3. Single element array
  // ============================================================
  describe("single element array", () => {
    it("should return the element for 0th percentile", () => {
      expect(percentile([42], 0)).toBe(42);
    });

    it("should return the element for 50th percentile", () => {
      expect(percentile([42], 50)).toBe(42);
    });

    it("should return the element for 100th percentile", () => {
      expect(percentile([42], 100)).toBe(42);
    });

    it("should return negative element for any percentile", () => {
      expect(percentile([-7], 25)).toBe(-7);
    });

    it("should return 0 for single zero element", () => {
      expect(percentile([0], 50)).toBe(0);
    });

    it("should return the element for a fractional percentile", () => {
      expect(percentile([99], 33)).toBe(99);
    });
  });

  // ============================================================
  // 4. Two element array
  // ============================================================
  describe("two element array", () => {
    it("should return first element for 0th percentile", () => {
      expect(percentile([1, 2], 0)).toBe(1);
    });

    it("should return last element for 100th percentile", () => {
      expect(percentile([1, 2], 100)).toBe(2);
    });

    it("should interpolate at 50th percentile", () => {
      expect(percentile([1, 2], 50)).toBe(1.5);
    });

    it("should interpolate at 25th percentile", () => {
      expect(percentile([1, 2], 25)).toBe(1.25);
    });

    it("should interpolate at 75th percentile", () => {
      expect(percentile([1, 2], 75)).toBe(1.75);
    });

    it("should handle two element unsorted", () => {
      expect(percentile([10, 2], 50)).toBe(6);
    });

    it("should interpolate at 10th percentile", () => {
      expect(percentile([0, 100], 10)).toBe(10);
    });

    it("should interpolate at 90th percentile", () => {
      expect(percentile([0, 100], 90)).toBe(90);
    });
  });

  // ============================================================
  // 5. Three element array
  // ============================================================
  describe("three element array", () => {
    it("should return min for 0th percentile", () => {
      expect(percentile([1, 2, 3], 0)).toBe(1);
    });

    it("should return median for 50th percentile", () => {
      expect(percentile([1, 2, 3], 50)).toBe(2);
    });

    it("should return max for 100th percentile", () => {
      expect(percentile([1, 2, 3], 100)).toBe(3);
    });

    it("should interpolate at 25th percentile", () => {
      expect(percentile([1, 2, 3], 25)).toBe(1.5);
    });

    it("should interpolate at 75th percentile", () => {
      expect(percentile([1, 2, 3], 75)).toBe(2.5);
    });
  });

  // ============================================================
  // 6. Boundary percentiles (0 and 100)
  // ============================================================
  describe("boundary percentiles", () => {
    it("should return minimum for 0th percentile", () => {
      expect(percentile([5, 3, 1, 4, 2], 0)).toBe(1);
    });

    it("should return maximum for 100th percentile", () => {
      expect(percentile([5, 3, 1, 4, 2], 100)).toBe(5);
    });

    it("should return min of large array for 0th percentile", () => {
      const arr = Array.from({ length: 100 }, (_, i) => i + 1);
      expect(percentile(arr, 0)).toBe(1);
    });

    it("should return max of large array for 100th percentile", () => {
      const arr = Array.from({ length: 100 }, (_, i) => i + 1);
      expect(percentile(arr, 100)).toBe(100);
    });

    it("should return min of negative array for 0th percentile", () => {
      expect(percentile([-100, -50, -10, -1], 0)).toBe(-100);
    });

    it("should return max of negative array for 100th percentile", () => {
      expect(percentile([-100, -50, -10, -1], 100)).toBe(-1);
    });
  });

  // ============================================================
  // 7. Unsorted input arrays
  // ============================================================
  describe("unsorted input arrays", () => {
    it("should sort before calculating percentile", () => {
      expect(percentile([5, 1, 3, 2, 4], 50)).toBe(3);
    });

    it("should handle reverse sorted array", () => {
      expect(percentile([5, 4, 3, 2, 1], 50)).toBe(3);
    });

    it("should handle already sorted array", () => {
      expect(percentile([1, 2, 3, 4, 5], 50)).toBe(3);
    });

    it("should handle random order with even length", () => {
      expect(percentile([8, 2, 6, 4], 50)).toBe(5);
    });

    it("should handle shuffled large array", () => {
      const sorted = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
      const shuffled = [60, 20, 90, 40, 10, 80, 50, 100, 30, 70];
      expect(percentile(shuffled, 0)).toBe(percentile(sorted, 0));
      expect(percentile(shuffled, 50)).toBe(percentile(sorted, 50));
      expect(percentile(shuffled, 100)).toBe(percentile(sorted, 100));
    });
  });

  // ============================================================
  // 8. Negative numbers
  // ============================================================
  describe("negative numbers", () => {
    it("should calculate 50th percentile of negative array", () => {
      expect(percentile([-5, -3, -1, 1, 3], 50)).toBe(-1);
    });

    it("should calculate 0th percentile of all-negative array", () => {
      expect(percentile([-10, -8, -6, -4, -2], 0)).toBe(-10);
    });

    it("should calculate 100th percentile of all-negative array", () => {
      expect(percentile([-10, -8, -6, -4, -2], 100)).toBe(-2);
    });

    it("should interpolate negative values at 25th percentile", () => {
      // sorted: [-10, -8, -6, -4, -2], index = 0.25*4 = 1
      expect(percentile([-10, -8, -6, -4, -2], 25)).toBe(-8);
    });

    it("should interpolate negative values at 50th percentile", () => {
      expect(percentile([-10, -8, -6, -4, -2], 50)).toBe(-6);
    });

    it("should calculate 75th percentile of negative array", () => {
      expect(percentile([-10, -8, -6, -4, -2], 75)).toBe(-4);
    });
  });

  // ============================================================
  // 9. Duplicate values
  // ============================================================
  describe("duplicate values", () => {
    it("should handle all identical values", () => {
      expect(percentile([5, 5, 5, 5, 5], 0)).toBe(5);
      expect(percentile([5, 5, 5, 5, 5], 50)).toBe(5);
      expect(percentile([5, 5, 5, 5, 5], 100)).toBe(5);
    });

    it("should handle pairs of duplicates", () => {
      expect(percentile([1, 1, 2, 2, 3, 3], 50)).toBe(2);
    });

    it("should handle many duplicates with one outlier", () => {
      expect(percentile([1, 1, 1, 1, 100], 0)).toBe(1);
      expect(percentile([1, 1, 1, 1, 100], 100)).toBe(100);
    });

    it("should handle duplicates at boundaries", () => {
      // sorted: [1, 1, 5, 5], index = 0.5*3 = 1.5
      // interpolation: 1 + (5-1)*0.5 = 3
      expect(percentile([5, 1, 1, 5], 50)).toBe(3);
    });

    it("should return consistent value for all-same array at any percentile", () => {
      for (let p = 0; p <= 100; p += 10) {
        expect(percentile([7, 7, 7], p)).toBe(7);
      }
    });

    it("should handle two distinct values repeated", () => {
      // sorted: [0, 0, 0, 10, 10, 10], 5 elements → length-1 = 5
      // P50: index = 2.5 → 0 + (10-0)*0.5 = 5
      expect(percentile([10, 0, 10, 0, 10, 0], 50)).toBe(5);
    });
  });

  // ============================================================
  // 10. Floating point values
  // ============================================================
  describe("floating point values", () => {
    it("should handle array of decimals", () => {
      // sorted: [0.1, 0.2, 0.3, 0.4, 0.5], P50 → index 2 → 0.3
      expect(percentile([0.1, 0.2, 0.3, 0.4, 0.5], 50)).toBeCloseTo(0.3);
    });

    it("should interpolate between decimals", () => {
      // sorted: [1.5, 2.5, 3.5], P25 → index 0.5
      // 1.5 + (2.5-1.5)*0.5 = 2.0
      expect(percentile([1.5, 2.5, 3.5], 25)).toBeCloseTo(2.0);
    });

    it("should handle very small decimals", () => {
      expect(
        percentile([0.001, 0.002, 0.003, 0.004, 0.005], 50),
      ).toBeCloseTo(0.003);
    });

    it("should handle mixed integers and floats", () => {
      // sorted: [1, 1.5, 2, 2.5, 3], P50 → index 2 → 2
      expect(percentile([2, 1, 2.5, 1.5, 3], 50)).toBe(2);
    });

    it("should handle negative decimals", () => {
      // sorted: [-2.5, -1.5, -0.5, 0.5, 1.5], P50 → -0.5
      expect(percentile([-0.5, 1.5, -2.5, 0.5, -1.5], 50)).toBeCloseTo(-0.5);
    });
  });

  // ============================================================
  // 11. Interpolation accuracy
  // ============================================================
  describe("interpolation accuracy", () => {
    it("should interpolate at 10th percentile of [1,2,3,4,5]", () => {
      // index = 0.1*4 = 0.4 → 1 + (2-1)*0.4 = 1.4
      expect(percentile([1, 2, 3, 4, 5], 10)).toBeCloseTo(1.4);
    });

    it("should interpolate at 20th percentile of [1,2,3,4,5]", () => {
      // index = 0.2*4 = 0.8 → 1 + (2-1)*0.8 = 1.8
      expect(percentile([1, 2, 3, 4, 5], 20)).toBeCloseTo(1.8);
    });

    it("should interpolate at 30th percentile of [1,2,3,4,5]", () => {
      // index = 0.3*4 = 1.2 → 2 + (3-2)*0.2 = 2.2
      expect(percentile([1, 2, 3, 4, 5], 30)).toBeCloseTo(2.2);
    });

    it("should interpolate at 40th percentile of [1,2,3,4,5]", () => {
      // index = 0.4*4 = 1.6 → 2 + (3-2)*0.6 = 2.6
      expect(percentile([1, 2, 3, 4, 5], 40)).toBeCloseTo(2.6);
    });

    it("should interpolate at 60th percentile of [1,2,3,4,5]", () => {
      // index = 0.6*4 = 2.4 → 3 + (4-3)*0.4 = 3.4
      expect(percentile([1, 2, 3, 4, 5], 60)).toBeCloseTo(3.4);
    });

    it("should interpolate at 70th percentile of [1,2,3,4,5]", () => {
      // index = 0.7*4 = 2.8 → 3 + (4-3)*0.8 = 3.8
      expect(percentile([1, 2, 3, 4, 5], 70)).toBeCloseTo(3.8);
    });

    it("should interpolate at 80th percentile of [1,2,3,4,5]", () => {
      // index = 0.8*4 = 3.2 → 4 + (5-4)*0.2 = 4.2
      expect(percentile([1, 2, 3, 4, 5], 80)).toBeCloseTo(4.2);
    });

    it("should interpolate at 90th percentile of [1,2,3,4,5]", () => {
      // index = 0.9*4 = 3.6 → 4 + (5-4)*0.6 = 4.6
      expect(percentile([1, 2, 3, 4, 5], 90)).toBeCloseTo(4.6);
    });

    it("should interpolate correctly with large gaps", () => {
      // sorted: [0, 1000], P50 → 500
      expect(percentile([0, 1000], 50)).toBe(500);
    });

    it("should interpolate at 33rd percentile of [0, 100]", () => {
      // index = 0.33*1 = 0.33 → 0 + 100*0.33 = 33
      expect(percentile([0, 100], 33)).toBeCloseTo(33);
    });

    it("should interpolate at 67th percentile of [0, 100]", () => {
      expect(percentile([0, 100], 67)).toBeCloseTo(67);
    });
  });

  // ============================================================
  // 12. Ten-element array (every 10th percentile)
  // ============================================================
  describe("ten-element array percentiles", () => {
    const arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    it("should return 10 for P0", () => {
      expect(percentile(arr, 0)).toBe(10);
    });

    it("should return 19 for P10", () => {
      // index = 0.1*9 = 0.9 → 10 + (20-10)*0.9 = 19
      expect(percentile(arr, 10)).toBeCloseTo(19);
    });

    it("should return 28 for P20", () => {
      // index = 0.2*9 = 1.8 → 20 + (30-20)*0.8 = 28
      expect(percentile(arr, 20)).toBeCloseTo(28);
    });

    it("should return 37 for P30", () => {
      // index = 0.3*9 = 2.7 → 30 + (40-30)*0.7 = 37
      expect(percentile(arr, 30)).toBeCloseTo(37);
    });

    it("should return 46 for P40", () => {
      // index = 0.4*9 = 3.6 → 40 + (50-40)*0.6 = 46
      expect(percentile(arr, 40)).toBeCloseTo(46);
    });

    it("should return 55 for P50", () => {
      // index = 0.5*9 = 4.5 → 50 + (60-50)*0.5 = 55
      expect(percentile(arr, 50)).toBe(55);
    });

    it("should return 64 for P60", () => {
      // index = 0.6*9 = 5.4 → 60 + (70-60)*0.4 = 64
      expect(percentile(arr, 60)).toBeCloseTo(64);
    });

    it("should return 73 for P70", () => {
      // index = 0.7*9 = 6.3 → 70 + (80-70)*0.3 = 73
      expect(percentile(arr, 70)).toBeCloseTo(73);
    });

    it("should return 82 for P80", () => {
      // index = 0.8*9 = 7.2 → 80 + (90-80)*0.2 = 82
      expect(percentile(arr, 80)).toBeCloseTo(82);
    });

    it("should return 91 for P90", () => {
      // index = 0.9*9 = 8.1 → 90 + (100-90)*0.1 = 91
      expect(percentile(arr, 90)).toBeCloseTo(91);
    });

    it("should return 100 for P100", () => {
      expect(percentile(arr, 100)).toBe(100);
    });
  });

  // ============================================================
  // 13. Large numbers
  // ============================================================
  describe("large numbers", () => {
    it("should handle millions", () => {
      expect(percentile([1000000, 2000000, 3000000], 50)).toBe(2000000);
    });

    it("should handle very large values", () => {
      expect(percentile([1e10, 2e10, 3e10, 4e10, 5e10], 50)).toBe(3e10);
    });

    it("should interpolate large values correctly", () => {
      // sorted: [1e6, 2e6], P50 → 1.5e6
      expect(percentile([1e6, 2e6], 50)).toBe(1.5e6);
    });
  });

  // ============================================================
  // 14. Mixed positive and negative
  // ============================================================
  describe("mixed positive and negative", () => {
    it("should calculate 50th percentile spanning zero", () => {
      // sorted: [-2, -1, 0, 1, 2], P50 → 0
      expect(percentile([-2, -1, 0, 1, 2], 50)).toBe(0);
    });

    it("should calculate 25th percentile spanning zero", () => {
      // sorted: [-2, -1, 0, 1, 2], P25 → index 1 → -1
      expect(percentile([-2, -1, 0, 1, 2], 25)).toBe(-1);
    });

    it("should calculate 75th percentile spanning zero", () => {
      expect(percentile([-2, -1, 0, 1, 2], 75)).toBe(1);
    });

    it("should interpolate across zero boundary", () => {
      // sorted: [-1, 1], P50 → -1 + (1-(-1))*0.5 = 0
      expect(percentile([-1, 1], 50)).toBe(0);
    });

    it("should handle wide range of mixed values", () => {
      // sorted: [-1000, -100, 0, 100, 1000]
      expect(percentile([0, 1000, -1000, 100, -100], 50)).toBe(0);
    });
  });

  // ============================================================
  // 15. Even-length arrays
  // ============================================================
  describe("even-length arrays", () => {
    it("should handle 4-element array at P50", () => {
      // sorted: [1,2,3,4], index = 0.5*3 = 1.5 → 2 + (3-2)*0.5 = 2.5
      expect(percentile([1, 2, 3, 4], 50)).toBe(2.5);
    });

    it("should handle 6-element array at P50", () => {
      // sorted: [1,2,3,4,5,6], index = 0.5*5 = 2.5 → 3 + (4-3)*0.5 = 3.5
      expect(percentile([1, 2, 3, 4, 5, 6], 50)).toBe(3.5);
    });

    it("should handle 4-element array at P25", () => {
      // sorted: [1,2,3,4], index = 0.25*3 = 0.75 → 1 + (2-1)*0.75 = 1.75
      expect(percentile([1, 2, 3, 4], 25)).toBeCloseTo(1.75);
    });

    it("should handle 4-element array at P75", () => {
      // sorted: [1,2,3,4], index = 0.75*3 = 2.25 → 3 + (4-3)*0.25 = 3.25
      expect(percentile([1, 2, 3, 4], 75)).toBeCloseTo(3.25);
    });
  });

  // ============================================================
  // 16. Odd-length arrays
  // ============================================================
  describe("odd-length arrays", () => {
    it("should handle 7-element array at P50", () => {
      // sorted: [1,2,3,4,5,6,7], index = 0.5*6 = 3 → 4
      expect(percentile([1, 2, 3, 4, 5, 6, 7], 50)).toBe(4);
    });

    it("should handle 9-element array at P50", () => {
      // sorted: [1..9], index = 0.5*8 = 4 → 5
      expect(percentile([1, 2, 3, 4, 5, 6, 7, 8, 9], 50)).toBe(5);
    });

    it("should handle 7-element array at P25", () => {
      // sorted: [1,2,3,4,5,6,7], index = 0.25*6 = 1.5 → 2 + (3-2)*0.5 = 2.5
      expect(percentile([1, 2, 3, 4, 5, 6, 7], 25)).toBe(2.5);
    });
  });

  // ============================================================
  // 17. Monotonicity property
  // ============================================================
  describe("monotonicity", () => {
    it("should be non-decreasing as percentile increases", () => {
      const arr = [15, 3, 8, 22, 11, 6, 19, 1, 25, 13];
      let prev = percentile(arr, 0);
      for (let p = 1; p <= 100; p++) {
        const curr = percentile(arr, p);
        expect(curr).toBeGreaterThanOrEqual(prev);
        prev = curr;
      }
    });

    it("should be monotonic for array with duplicates", () => {
      const arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
      let prev = percentile(arr, 0);
      for (let p = 5; p <= 100; p += 5) {
        const curr = percentile(arr, p);
        expect(curr).toBeGreaterThanOrEqual(prev);
        prev = curr;
      }
    });
  });

  // ============================================================
  // 18. Bounds property
  // ============================================================
  describe("bounds property", () => {
    it("should always return value between min and max", () => {
      const arr = [50, 10, 90, 30, 70, 20, 80, 40, 60, 100];
      const min = 10;
      const max = 100;
      for (let p = 0; p <= 100; p += 5) {
        const val = percentile(arr, p);
        expect(val).toBeGreaterThanOrEqual(min);
        expect(val).toBeLessThanOrEqual(max);
      }
    });

    it("P0 should equal min and P100 should equal max", () => {
      const arr = [42, 17, 88, 5, 73, 29, 61];
      const sorted = [...arr].sort((a, b) => a - b);
      expect(percentile(arr, 0)).toBe(sorted[0]);
      expect(percentile(arr, 100)).toBe(sorted[sorted.length - 1]);
    });

    it("should bound results for negative arrays", () => {
      const arr = [-50, -10, -90, -30, -70];
      for (let p = 0; p <= 100; p += 10) {
        const val = percentile(arr, p);
        expect(val).toBeGreaterThanOrEqual(-90);
        expect(val).toBeLessThanOrEqual(-10);
      }
    });
  });

  // ============================================================
  // 19. Immutability (input not mutated)
  // ============================================================
  describe("immutability", () => {
    it("should not mutate the original array", () => {
      const arr = [5, 3, 1, 4, 2];
      const copy = [...arr];
      percentile(arr, 50);
      expect(arr).toEqual(copy);
    });

    it("should not mutate when called multiple times", () => {
      const arr = [10, 8, 6, 4, 2];
      const copy = [...arr];
      percentile(arr, 25);
      percentile(arr, 50);
      percentile(arr, 75);
      expect(arr).toEqual(copy);
    });
  });

  // ============================================================
  // 20. Large arrays
  // ============================================================
  describe("large arrays", () => {
    it("should handle 100-element sequential array", () => {
      const arr = Array.from({ length: 100 }, (_, i) => i + 1);
      expect(percentile(arr, 0)).toBe(1);
      expect(percentile(arr, 50)).toBe(50.5);
      expect(percentile(arr, 100)).toBe(100);
    });

    it("should handle 1000-element array", () => {
      const arr = Array.from({ length: 1000 }, (_, i) => i);
      expect(percentile(arr, 0)).toBe(0);
      expect(percentile(arr, 100)).toBe(999);
    });

    it("should handle shuffled 100-element array", () => {
      const sorted = Array.from({ length: 100 }, (_, i) => i);
      // Deterministic shuffle using simple swap pattern
      const shuffled = [...sorted];
      for (let i = 0; i < shuffled.length; i++) {
        const j = (i * 37 + 13) % shuffled.length;
        [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
      }
      expect(percentile(shuffled, 0)).toBe(0);
      expect(percentile(shuffled, 100)).toBe(99);
    });

    it("should calculate P50 of large array correctly", () => {
      // 1..101 (101 elements, odd), P50 → index 50 → 51
      const arr = Array.from({ length: 101 }, (_, i) => i + 1);
      expect(percentile(arr, 50)).toBe(51);
    });
  });

  // ============================================================
  // 21. Infinity values
  // ============================================================
  describe("infinity values", () => {
    it("should handle positive infinity in array", () => {
      expect(percentile([1, 2, Number.POSITIVE_INFINITY], 100)).toBe(
        Number.POSITIVE_INFINITY,
      );
    });

    it("should handle negative infinity in array", () => {
      expect(percentile([Number.NEGATIVE_INFINITY, 1, 2], 0)).toBe(
        Number.NEGATIVE_INFINITY,
      );
    });

    it("should handle both infinities", () => {
      const result = percentile(
        [Number.NEGATIVE_INFINITY, 0, Number.POSITIVE_INFINITY],
        50,
      );
      expect(result).toBe(0);
    });

    it("should return Infinity for P100 with positive infinity", () => {
      expect(percentile([1, 2, 3, Number.POSITIVE_INFINITY], 100)).toBe(
        Number.POSITIVE_INFINITY,
      );
    });
  });

  // ============================================================
  // 22. Specific percentile values
  // ============================================================
  describe("specific percentile values", () => {
    it("should calculate P1", () => {
      // [1..101], P1 → index = 0.01*100 = 1 → 2
      const arr = Array.from({ length: 101 }, (_, i) => i + 1);
      expect(percentile(arr, 1)).toBe(2);
    });

    it("should calculate P5", () => {
      // [1..101], P5 → index = 0.05*100 = 5 → 6
      const arr = Array.from({ length: 101 }, (_, i) => i + 1);
      expect(percentile(arr, 5)).toBe(6);
    });

    it("should calculate P95", () => {
      // [1..101], P95 → index = 0.95*100 = 95 → 96
      const arr = Array.from({ length: 101 }, (_, i) => i + 1);
      expect(percentile(arr, 95)).toBe(96);
    });

    it("should calculate P99", () => {
      // [1..101], P99 → index = 0.99*100 = 99 → 100
      const arr = Array.from({ length: 101 }, (_, i) => i + 1);
      expect(percentile(arr, 99)).toBe(100);
    });

    it("should calculate P33 of [0, 25, 50, 75, 100]", () => {
      // index = 0.33*4 = 1.32 → 25 + (50-25)*0.32 = 33
      expect(percentile([0, 25, 50, 75, 100], 33)).toBeCloseTo(33);
    });

    it("should calculate P66 of [0, 25, 50, 75, 100]", () => {
      // index = 0.66*4 = 2.64 → 50 + (75-50)*0.64 = 66
      expect(percentile([0, 25, 50, 75, 100], 66)).toBeCloseTo(66);
    });
  });

  // ============================================================
  // 23. Quartile consistency
  // ============================================================
  describe("quartile consistency", () => {
    it("Q1 <= Q2 <= Q3 for random array", () => {
      const arr = [42, 7, 93, 15, 68, 31, 84, 56, 2, 77];
      const q1 = percentile(arr, 25);
      const q2 = percentile(arr, 50);
      const q3 = percentile(arr, 75);
      expect(q1).toBeLessThanOrEqual(q2);
      expect(q2).toBeLessThanOrEqual(q3);
    });

    it("IQR should be non-negative", () => {
      const arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
      const q1 = percentile(arr, 25);
      const q3 = percentile(arr, 75);
      expect(q3 - q1).toBeGreaterThanOrEqual(0);
    });

    it("deciles should be ordered", () => {
      const arr = [5, 15, 25, 35, 45, 55, 65, 75, 85, 95];
      const deciles = [10, 20, 30, 40, 50, 60, 70, 80, 90].map((p) =>
        percentile(arr, p),
      );
      for (let i = 1; i < deciles.length; i++) {
        expect(deciles[i]).toBeGreaterThanOrEqual(deciles[i - 1]);
      }
    });
  });

  // ============================================================
  // 24. Symmetry properties
  // ============================================================
  describe("symmetry properties", () => {
    it("P50 of symmetric array should equal center value", () => {
      // sorted: [1,2,3,4,5], symmetric around 3
      expect(percentile([1, 2, 3, 4, 5], 50)).toBe(3);
    });

    it("P25 and P75 should be equidistant from median for uniform array", () => {
      const arr = [0, 25, 50, 75, 100];
      const q1 = percentile(arr, 25);
      const q2 = percentile(arr, 50);
      const q3 = percentile(arr, 75);
      expect(q2 - q1).toBeCloseTo(q3 - q2);
    });

    it("P(x) + P(100-x) should relate to min+max for uniform data", () => {
      const arr = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
      // For uniform [0..100], P(x) ≈ x
      expect(percentile(arr, 20) + percentile(arr, 80)).toBeCloseTo(100);
      expect(percentile(arr, 30) + percentile(arr, 70)).toBeCloseTo(100);
    });
  });

  // ============================================================
  // 25. Special distributions
  // ============================================================
  describe("special distributions", () => {
    it("should handle arithmetic progression", () => {
      // [0, 10, 20, 30, ..., 100]
      const arr = Array.from({ length: 11 }, (_, i) => i * 10);
      expect(percentile(arr, 50)).toBe(50);
      expect(percentile(arr, 25)).toBe(25);
      expect(percentile(arr, 75)).toBe(75);
    });

    it("should handle geometric-like progression", () => {
      const arr = [1, 2, 4, 8, 16];
      expect(percentile(arr, 0)).toBe(1);
      expect(percentile(arr, 50)).toBe(4);
      expect(percentile(arr, 100)).toBe(16);
    });

    it("should handle power-of-two array", () => {
      const arr = [1, 2, 4, 8, 16, 32, 64, 128, 256];
      expect(percentile(arr, 0)).toBe(1);
      expect(percentile(arr, 100)).toBe(256);
      expect(percentile(arr, 50)).toBe(16);
    });

    it("should handle array with zeros", () => {
      expect(percentile([0, 0, 0, 0, 1], 50)).toBe(0);
      expect(percentile([0, 0, 0, 0, 1], 100)).toBe(1);
    });

    it("should handle single large outlier", () => {
      const arr = [1, 2, 3, 4, 1000];
      expect(percentile(arr, 0)).toBe(1);
      expect(percentile(arr, 50)).toBe(3);
      expect(percentile(arr, 100)).toBe(1000);
    });
  });

  // ============================================================
  // 26. Exact index vs interpolation
  // ============================================================
  describe("exact index vs interpolation", () => {
    it("should return exact value when index is integer (5 elements, P25)", () => {
      // 5 elements, P25: index = 0.25*4 = 1.0 (exact)
      expect(percentile([10, 20, 30, 40, 50], 25)).toBe(20);
    });

    it("should return exact value when index is integer (5 elements, P75)", () => {
      // 5 elements, P75: index = 0.75*4 = 3.0 (exact)
      expect(percentile([10, 20, 30, 40, 50], 75)).toBe(40);
    });

    it("should interpolate when index is not integer (4 elements, P25)", () => {
      // 4 elements, P25: index = 0.25*3 = 0.75 (not exact)
      // 10 + (20-10)*0.75 = 17.5
      expect(percentile([10, 20, 30, 40], 25)).toBeCloseTo(17.5);
    });

    it("should interpolate when index is not integer (4 elements, P75)", () => {
      // 4 elements, P75: index = 0.75*3 = 2.25
      // 30 + (40-30)*0.25 = 32.5
      expect(percentile([10, 20, 30, 40], 75)).toBeCloseTo(32.5);
    });

    it("should return exact for P0 regardless of array length", () => {
      expect(percentile([7, 14, 21], 0)).toBe(7);
      expect(percentile([7, 14, 21, 28], 0)).toBe(7);
    });

    it("should return exact for P100 regardless of array length", () => {
      expect(percentile([7, 14, 21], 100)).toBe(21);
      expect(percentile([7, 14, 21, 28], 100)).toBe(28);
    });
  });

  // ============================================================
  // 27. Consistency across multiple calls
  // ============================================================
  describe("consistency", () => {
    it("should return same result for identical calls", () => {
      const arr = [5, 3, 8, 1, 9, 4, 7, 2, 6, 10];
      const result1 = percentile(arr, 42);
      const result2 = percentile(arr, 42);
      expect(result1).toBe(result2);
    });

    it("should return same result regardless of input order", () => {
      const a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
      const b = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
      const c = [5, 10, 3, 8, 1, 6, 9, 2, 7, 4];
      expect(percentile(a, 35)).toBe(percentile(b, 35));
      expect(percentile(b, 35)).toBe(percentile(c, 35));
    });

    it("should produce identical results for array permutations", () => {
      const perm1 = [4, 1, 3, 2, 5];
      const perm2 = [2, 5, 1, 4, 3];
      const perm3 = [3, 2, 5, 1, 4];
      for (const p of [0, 10, 25, 50, 75, 90, 100]) {
        const v1 = percentile(perm1, p);
        const v2 = percentile(perm2, p);
        const v3 = percentile(perm3, p);
        expect(v1).toBe(v2);
        expect(v2).toBe(v3);
      }
    });
  });

  // ============================================================
  // 28. Weighted interpolation verification
  // ============================================================
  describe("weighted interpolation", () => {
    it("should produce linear interpolation for uniform spacing", () => {
      // [0, 100] array: P(x) should equal x
      for (let p = 0; p <= 100; p += 10) {
        expect(percentile([0, 100], p)).toBeCloseTo(p);
      }
    });

    it("should interpolate correctly with non-uniform spacing", () => {
      // sorted: [0, 1, 10], 3 elements
      // P25: index = 0.5 → 0 + (1-0)*0.5 = 0.5
      // P75: index = 1.5 → 1 + (10-1)*0.5 = 5.5
      expect(percentile([0, 1, 10], 25)).toBeCloseTo(0.5);
      expect(percentile([0, 1, 10], 75)).toBeCloseTo(5.5);
    });

    it("should weight correctly at 1/3 between elements", () => {
      // sorted: [0, 30], P(100/3) → index = (1/3)*1 ≈ 0.333
      // 0 + 30*0.333 ≈ 10
      const result = percentile([0, 30], 100 / 3);
      expect(result).toBeCloseTo(10);
    });
  });

  // ============================================================
  // 29. Edge-case array values
  // ============================================================
  describe("edge-case values", () => {
    it("should handle Number.MAX_SAFE_INTEGER", () => {
      const arr = [0, Number.MAX_SAFE_INTEGER];
      expect(percentile(arr, 0)).toBe(0);
      expect(percentile(arr, 100)).toBe(Number.MAX_SAFE_INTEGER);
    });

    it("should handle Number.MIN_SAFE_INTEGER", () => {
      const arr = [Number.MIN_SAFE_INTEGER, 0];
      expect(percentile(arr, 0)).toBe(Number.MIN_SAFE_INTEGER);
      expect(percentile(arr, 100)).toBe(0);
    });

    it("should handle very small positive numbers", () => {
      const arr = [1e-10, 2e-10, 3e-10, 4e-10, 5e-10];
      expect(percentile(arr, 50)).toBeCloseTo(3e-10);
    });

    it("should handle mix of tiny and large numbers", () => {
      const arr = [0.001, 1000000];
      expect(percentile(arr, 50)).toBeCloseTo(500000.0005);
    });
  });

  // ============================================================
  // 30. Real-world-like data scenarios
  // ============================================================
  describe("real-world-like scenarios", () => {
    it("should calculate test score percentiles", () => {
      const scores = [65, 70, 75, 80, 85, 90, 95, 100, 55, 60];
      // sorted: [55,60,65,70,75,80,85,90,95,100]
      expect(percentile(scores, 0)).toBe(55);
      expect(percentile(scores, 100)).toBe(100);
    });

    it("should calculate temperature percentiles", () => {
      const temps = [15, 18, 20, 22, 25, 28, 30, 32, 35, 38, 40, 42];
      expect(percentile(temps, 0)).toBe(15);
      expect(percentile(temps, 100)).toBe(42);
      const median = percentile(temps, 50);
      expect(median).toBeGreaterThan(15);
      expect(median).toBeLessThan(42);
    });

    it("should handle salary-like skewed data", () => {
      const salaries = [
        30000, 35000, 40000, 45000, 50000, 55000, 60000, 70000, 100000,
        500000,
      ];
      const p90 = percentile(salaries, 90);
      // index = 0.9*9 = 8.1 → 100000 + (500000-100000)*0.1 = 140000
      expect(p90).toBeCloseTo(140000);
    });

    it("should calculate response time percentiles", () => {
      const times = [
        10, 12, 15, 18, 20, 22, 25, 30, 50, 100, 200, 500, 1000,
      ];
      const p50 = percentile(times, 50);
      const p95 = percentile(times, 95);
      const p99 = percentile(times, 99);
      expect(p50).toBeLessThan(p95);
      expect(p95).toBeLessThan(p99);
    });
  });

  // ============================================================
  // 31. Consecutive integer percentiles
  // ============================================================
  describe("consecutive integer percentiles", () => {
    it("should handle all integer percentiles from 0 to 100 on 5-element array", () => {
      const arr = [10, 20, 30, 40, 50];
      for (let p = 0; p <= 100; p++) {
        const val = percentile(arr, p);
        expect(val).toBeGreaterThanOrEqual(10);
        expect(val).toBeLessThanOrEqual(50);
        expect(typeof val).toBe("number");
        expect(Number.isNaN(val)).toBe(false);
      }
    });
  });
});
