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

  test("should generate an array of consecutive numbers from 0 to 5", () => {
    const result = range(5);
    expect(result).toEqual([0, 1, 2, 3, 4]);
  });
});
