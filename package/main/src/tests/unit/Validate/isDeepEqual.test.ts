import { isDeepEqual } from "@/Validate/isDeepEqual";

describe("isDeepEqual", () => {
  describe("basic type comparison", () => {
    it("should compare primitive values correctly", () => {
      expect(isDeepEqual(1, 1)).toBe(true);
      expect(isDeepEqual("test", "test")).toBe(true);
      expect(isDeepEqual(true, true)).toBe(true);
      expect(isDeepEqual(null, null)).toBe(true);
      expect(isDeepEqual(undefined, undefined)).toBe(true);
    });

    it("should return false for different primitive values", () => {
      expect(isDeepEqual(1, 2)).toBe(false);
      expect(isDeepEqual("test", "other")).toBe(false);
      expect(isDeepEqual(true, false)).toBe(false);
      expect(isDeepEqual(null, undefined)).toBe(false);
    });

    it("should use Object.is semantics", () => {
      expect(isDeepEqual(Number.NaN, Number.NaN)).toBe(true);
      expect(isDeepEqual(-0, +0)).toBe(false);
    });

    it("should return false for different types", () => {
      expect(isDeepEqual(1, "1")).toBe(false);
      expect(isDeepEqual(0, false)).toBe(false);
      expect(isDeepEqual([], {})).toBe(false);
    });
  });

  describe("array comparison", () => {
    it("should compare arrays with same elements", () => {
      expect(isDeepEqual([1, 2, 3], [1, 2, 3])).toBe(true);
      expect(isDeepEqual([], [])).toBe(true);
      expect(isDeepEqual([1, [2, 3]], [1, [2, 3]])).toBe(true);
    });

    it("should return false for different arrays", () => {
      expect(isDeepEqual([1, 2, 3], [1, 2, 4])).toBe(false);
      expect(isDeepEqual([1, 2, 3], [1, 2, 3, 4])).toBe(false);
      expect(isDeepEqual([1, 2, 3], [3, 2, 1])).toBe(false);
    });

    it("should handle strictOrder option", () => {
      expect(isDeepEqual([1, 2, 3], [3, 2, 1], { strictOrder: true })).toBe(
        false,
      );
      expect(isDeepEqual([1, 2, 3], [3, 2, 1], { strictOrder: false })).toBe(
        false,
      );
    });

    it("should handle empty stack condition", () => {
      const result = isDeepEqual([], []);
      expect(result).toBe(true);
    });
  });

  describe("object comparison", () => {
    it("should compare objects with same properties", () => {
      expect(isDeepEqual({ a: 1, b: 2 }, { a: 1, b: 2 })).toBe(true);
      expect(isDeepEqual({ a: 1, b: 2 }, { b: 2, a: 1 })).toBe(true);
      expect(isDeepEqual({}, {})).toBe(true);
    });

    it("should return false for different objects", () => {
      expect(isDeepEqual({ a: 1, b: 2 }, { a: 1, b: 3 })).toBe(false);
      expect(isDeepEqual({ a: 1, b: 2 }, { a: 1 })).toBe(false);
      expect(isDeepEqual({ a: 1 }, { a: 1, b: 2 })).toBe(false);
    });

    it("should handle nested objects", () => {
      expect(isDeepEqual({ a: { b: 1 } }, { a: { b: 1 } })).toBe(true);
      expect(isDeepEqual({ a: { b: 1 } }, { a: { b: 2 } })).toBe(false);
    });
  });

  describe("Date object comparison", () => {
    it("should compare dates with same time", () => {
      const date1 = new Date("2023-01-01");
      const date2 = new Date("2023-01-01");
      expect(isDeepEqual(date1, date2)).toBe(true);
    });

    it("should return false for different dates", () => {
      const date1 = new Date("2023-01-01");
      const date2 = new Date("2023-01-02");
      expect(isDeepEqual(date1, date2)).toBe(false);
    });
  });

  describe("RegExp object comparison", () => {
    it("should compare RegExp with same pattern and flags", () => {
      expect(isDeepEqual(/abc/g, /abc/g)).toBe(true);
      expect(isDeepEqual(/abc/i, /abc/i)).toBe(true);
    });

    it("should return false for different RegExp", () => {
      expect(isDeepEqual(/abc/g, /abc/i)).toBe(false);
      expect(isDeepEqual(/abc/, /def/)).toBe(false);
    });
  });

  describe("Set object comparison", () => {
    it("should compare Sets with same elements", () => {
      expect(isDeepEqual(new Set([1, 2, 3]), new Set([1, 2, 3]))).toBe(true);
      expect(isDeepEqual(new Set([1, 2, 3]), new Set([3, 2, 1]))).toBe(true);
      expect(isDeepEqual(new Set(), new Set())).toBe(true);
    });

    it("should return false for different Sets", () => {
      expect(isDeepEqual(new Set([1, 2, 3]), new Set([1, 2, 4]))).toBe(false);
      expect(isDeepEqual(new Set([1, 2, 3]), new Set([1, 2, 3, 4]))).toBe(
        false,
      );
    });
  });

  describe("Map object comparison", () => {
    it("should compare Maps with same key-value pairs", () => {
      const map1 = new Map([
        ["a", 1],
        ["b", 2],
      ]);
      const map2 = new Map([
        ["a", 1],
        ["b", 2],
      ]);
      expect(isDeepEqual(map1, map2)).toBe(true);

      const map3 = new Map([
        ["a", 1],
        ["b", 2],
      ]);
      const map4 = new Map([
        ["b", 2],
        ["a", 1],
      ]);
      expect(isDeepEqual(map3, map4)).toBe(true);
    });

    it("should return false for different Maps", () => {
      const map1 = new Map([
        ["a", 1],
        ["b", 2],
      ]);
      const map2 = new Map([
        ["a", 1],
        ["b", 3],
      ]);
      expect(isDeepEqual(map1, map2)).toBe(false);

      const map3 = new Map([["a", 1]]);
      const map4 = new Map([
        ["a", 1],
        ["b", 2],
      ]);
      expect(isDeepEqual(map3, map4)).toBe(false);
    });
  });

  describe("TypedArray comparison", () => {
    it("should compare TypedArrays with same elements", () => {
      expect(
        isDeepEqual(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2, 3])),
      ).toBe(true);
      expect(
        isDeepEqual(new Int32Array([1, 2, 3]), new Int32Array([1, 2, 3])),
      ).toBe(true);
    });

    it("should return false for different TypedArrays", () => {
      expect(
        isDeepEqual(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2, 4])),
      ).toBe(false);
      expect(
        isDeepEqual(new Uint8Array([1, 2, 3]), new Uint8Array([1, 2, 3, 4])),
      ).toBe(false);
    });

    it("should return false for different TypedArray classes", () => {
      expect(
        isDeepEqual(new Uint8Array([1, 2, 3]), new Int8Array([1, 2, 3])),
      ).toBe(false);
    });
  });

  describe("complex cases", () => {
    it("should handle deeply nested objects", () => {
      const obj1 = {
        a: {
          b: {
            c: [1, 2, { d: "test" }],
          },
        },
      };
      const obj2 = {
        a: {
          b: {
            c: [1, 2, { d: "test" }],
          },
        },
      };
      expect(isDeepEqual(obj1, obj2)).toBe(true);
    });

    it("should handle mixed object types", () => {
      const obj1 = { a: 1, b: "test" };
      const obj2 = { a: 1, b: "test" };
      expect(isDeepEqual(obj1, obj2)).toBe(true);
    });
  });

  describe("edge cases", () => {
    it("should handle null comparisons", () => {
      expect(isDeepEqual(null, null)).toBe(true);
      expect(isDeepEqual(null, {})).toBe(false);
    });

    it("should handle undefined comparisons", () => {
      expect(isDeepEqual(undefined, undefined)).toBe(true);
      expect(isDeepEqual(undefined, {})).toBe(false);
    });

    it("should handle empty objects", () => {
      expect(isDeepEqual({}, {})).toBe(true);
      expect(isDeepEqual([], [])).toBe(true);
    });

    it("should return false for objects with different constructors", () => {
      class A {
        constructor(public value: number) {}
      }
      class B {
        constructor(public value: number) {}
      }
      expect(isDeepEqual(new A(1), new B(1))).toBe(false);
    });

    it("should handle object with missing keys", () => {
      const obj1 = { a: 1, b: 2 };
      const obj2 = { a: 1, c: 2 };
      expect(isDeepEqual(obj1, obj2)).toBe(false);
    });
  });
});
