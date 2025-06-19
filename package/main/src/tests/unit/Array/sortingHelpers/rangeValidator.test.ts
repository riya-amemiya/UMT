import { validateRange } from "@/Array/sortingHelpers/rangeValidator";

describe("validateRange function", () => {
  describe("valid range scenarios", () => {
    test("should validate normal range within bounds", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 1, 3);

      expect(result).toEqual({
        startIndex: 1,
        endIndex: 3,
        shouldSort: true,
      });
    });

    test("should handle full array range", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 0, 4);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 4,
        shouldSort: true,
      });
    });

    test("should handle single element range", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 2, 2);

      expect(result).toEqual({
        startIndex: 2,
        endIndex: 2,
        shouldSort: true,
      });
    });
  });

  describe("boundary adjustments", () => {
    test("should clamp negative start index to 0", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, -5, 2);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 2,
        shouldSort: true,
      });
    });

    test("should clamp end index beyond array length", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 1, 10);

      expect(result).toEqual({
        startIndex: 1,
        endIndex: 4,
        shouldSort: true,
      });
    });

    test("should clamp start index beyond array length", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 10, 15);

      expect(result).toEqual({
        startIndex: 4,
        endIndex: 4,
        shouldSort: true,
      });
    });

    test("should handle both indices out of bounds", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, -10, 20);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 4,
        shouldSort: true,
      });
    });
  });

  describe("edge cases", () => {
    test("should handle empty array", () => {
      const array: number[] = [];
      const result = validateRange(array, 0, 5);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: -1,
        shouldSort: false,
      });
    });

    test("should handle single element array", () => {
      const array = [42];
      const result = validateRange(array, 0, 0);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 0,
        shouldSort: true,
      });
    });

    test("should handle single element array with out of bounds indices", () => {
      const array = [42];
      const result = validateRange(array, -1, 5);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 0,
        shouldSort: true,
      });
    });
  });

  describe("reversed range handling", () => {
    test("should handle reversed indices (doesn't swap, takes min/max)", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 3, 1);

      expect(result).toEqual({
        startIndex: 3,
        endIndex: 3,
        shouldSort: true,
      });
    });

    test("should handle reversed indices with clamping", () => {
      const array = [1, 2, 3, 4, 5];
      const result = validateRange(array, 10, -5);

      expect(result).toEqual({
        startIndex: 4,
        endIndex: 4,
        shouldSort: true,
      });
    });
  });

  describe("shouldSort flag", () => {
    test("should return shouldSort false for empty arrays", () => {
      const array: number[] = [];
      const result = validateRange(array, 0, 0);

      expect(result.shouldSort).toBe(false);
    });

    test("should return shouldSort true for valid ranges", () => {
      const array = [1, 2, 3, 4, 5];

      expect(validateRange(array, 0, 4).shouldSort).toBe(true);
      expect(validateRange(array, 2, 2).shouldSort).toBe(true);
      expect(validateRange(array, 1, 3).shouldSort).toBe(true);
    });

    test("should return shouldSort true even after range adjustments", () => {
      const array = [1, 2, 3, 4, 5];

      expect(validateRange(array, -10, 20).shouldSort).toBe(true);
      expect(validateRange(array, 10, 15).shouldSort).toBe(true);
    });
  });

  describe("various array types", () => {
    test("should work with string arrays", () => {
      const array = ["a", "b", "c", "d"];
      const result = validateRange(array, 1, 2);

      expect(result).toEqual({
        startIndex: 1,
        endIndex: 2,
        shouldSort: true,
      });
    });

    test("should work with object arrays", () => {
      const array = [{ id: 1 }, { id: 2 }, { id: 3 }];
      const result = validateRange(array, 0, 1);

      expect(result).toEqual({
        startIndex: 0,
        endIndex: 1,
        shouldSort: true,
      });
    });

    test("should work with mixed type arrays", () => {
      const array = [1, "two", { three: 3 }, null, undefined];
      const result = validateRange(array, 2, 4);

      expect(result).toEqual({
        startIndex: 2,
        endIndex: 4,
        shouldSort: true,
      });
    });
  });

  describe("performance considerations", () => {
    test("should handle large arrays efficiently", () => {
      const array = new Array(10_000).fill(0).map((_, i) => i);
      const result = validateRange(array, 100, 500);

      expect(result).toEqual({
        startIndex: 100,
        endIndex: 500,
        shouldSort: true,
      });
    });

    test("should not modify original array", () => {
      const array = [1, 2, 3, 4, 5];
      const originalArray = [...array];

      validateRange(array, 1, 3);

      expect(array).toEqual(originalArray);
    });
  });

  describe("return type consistency", () => {
    test("should always return ValidatedSortRange interface", () => {
      const array = [1, 2, 3];
      const result = validateRange(array, 0, 2);

      expect(result).toHaveProperty("startIndex");
      expect(result).toHaveProperty("endIndex");
      expect(result).toHaveProperty("shouldSort");
      expect(typeof result.startIndex).toBe("number");
      expect(typeof result.endIndex).toBe("number");
      expect(typeof result.shouldSort).toBe("boolean");
    });

    test("should maintain type consistency across different scenarios", () => {
      const testCases = [
        { array: [], start: 0, end: 0 },
        { array: [1], start: 0, end: 0 },
        { array: [1, 2, 3], start: -5, end: 10 },
        { array: [1, 2, 3, 4, 5], start: 2, end: 1 },
      ];

      for (const { array, start, end } of testCases) {
        const result = validateRange(array, start, end);
        expect(typeof result.startIndex).toBe("number");
        expect(typeof result.endIndex).toBe("number");
        expect(typeof result.shouldSort).toBe("boolean");
      }
    });
  });
});
