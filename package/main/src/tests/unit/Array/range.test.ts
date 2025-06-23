import { range } from "@/Array/range";

describe("range", () => {
  test("should generate an array of consecutive numbers from -5 to 5", () => {
    const result = range(-5, 5);
    expect(result).toEqual([-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]);
  });

  test("should generate an array of consecutive numbers from 1 to 10", () => {
    const result = range(1, 10);
    expect(result).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9]);
  });

  test("should return an empty array when start equals end", () => {
    expect(range(0, 0)).toEqual([]);
    expect(range(5, 5)).toEqual([]);
  });

  test("should generate an array of consecutive numbers from 0 to start", () => {
    const result = range(5);
    expect(result).toEqual([0, 1, 2, 3, 4]);
  });

  test("should generate an array with specified step", () => {
    expect(range(0, 10, 2)).toEqual([0, 2, 4, 6, 8]);
    expect(range(0, 5, 0.5)).toEqual([0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5]);
  });

  test("should handle negative step", () => {
    expect(range(5, 0, -1)).toEqual([5, 4, 3, 2, 1]);
    expect(range(0, -5, -1)).toEqual([0, -1, -2, -3, -4]);
    expect(range(10, 5, -2)).toEqual([10, 8, 6]);
  });

  test("should handle step of zero", () => {
    expect(range(0, 5, 0)).toEqual([]); // Avoid infinite loop
  });

  test("should handle start greater than end", () => {
    expect(range(10, 5)).toEqual([]); // Returns empty array when start > end
  });

  test("should handle floating point steps", () => {
    expect(range(0, 1, 0.2)).toEqual([0, 0.2, 0.4, 0.6, 0.8]);
  });

  test("should handle single step from 0 to start", () => {
    expect(range(3, undefined, 2)).toEqual([0, 2]);
  });
});
