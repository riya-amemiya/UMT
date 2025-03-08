import { generateNumberArray } from "@/Array/generateNumberArray";

describe("generateNumberArray", () => {
  it("should generate an array with the specified length", () => {
    const result = generateNumberArray(5);
    expect(result).toEqual([0, 1, 2, 3, 4]);
  });

  it("should generate an array with specified min and max values", () => {
    const result = generateNumberArray(5, 10, 14);
    expect(result).toEqual([10, 11, 12, 13, 14]);
  });

  it("should return an empty array when length is 0", () => {
    const result = generateNumberArray(0);
    expect(result).toEqual([]);
  });

  it("should throw an error when min is greater than max", () => {
    expect(() => generateNumberArray(5, 10, 5)).toThrow(
      "min should be less than or equal to max",
    );
  });

  it("should return min value when length is 1", () => {
    const result = generateNumberArray(1, 10, 20);
    expect(result).toEqual([10]);
  });

  it("should generate an array with random values", () => {
    const result = generateNumberArray(5, 10, 14, true);
    expect(result).toHaveLength(5);
    result.forEach((item) => {
      expect(item).toBeGreaterThanOrEqual(10);
      expect(item).toBeLessThanOrEqual(14);
    });
  });
});
