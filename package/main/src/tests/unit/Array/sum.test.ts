import { sum } from "@/Array/sum";
describe("sum function", () => {
  it("should calculate sum of integer numbers", () => {
    expect(sum([1, 2, 3])).toBe(6);
    expect(sum([10, 20, 30])).toBe(60);
    expect(sum([-1, -2, -3])).toBe(-6);
    expect(sum([])).toBe(0);
  });

  it("should calculate sum of decimal numbers", () => {
    expect(sum([0.1, 0.2, 0.3])).toBe(0.6);
    expect(sum([1.1, 2.2, 3.3])).toBe(6.6);
    expect(sum([0.1, 0.02, 0.003])).toBe(0.123);
  });

  it("should handle mixed integer and decimal numbers", () => {
    expect(sum([1, 2.5, 3.7])).toBe(7.2);
    expect(sum([-1.5, 2, -3.7])).toBe(-3.2);
  });

  it("should handle large numbers", () => {
    const largeNum = 1_000_000_000; // 10^9
    expect(sum([largeNum, largeNum])).toBe(2 * largeNum);
    expect(sum([largeNum, -largeNum])).toBe(0);
    expect(sum([largeNum, 1, -1])).toBe(largeNum);

    const mediumNum = 1_000_000; // 10^6
    const count = 1000;
    const arr = new Array(count).fill(mediumNum);
    expect(sum(arr)).toBe(mediumNum * count);
  });

  it("should handle very small numbers", () => {
    const smallNum = Number.MIN_VALUE;
    expect(sum([smallNum, smallNum])).toBe(2 * smallNum);
    expect(sum([smallNum, -smallNum])).toBe(0);
  });

  it("should handle JavaScript number precision", () => {
    // Testing number precision handling
    expect(sum([0.1, 0.2])).toBeCloseTo(0.3, 10);
    expect(sum([0.1, 0.2, 0.3, 0.4, 0.5])).toBeCloseTo(1.5, 10);
  });

  it("should maintain precision for many decimal places", () => {
    expect(sum([0.0001, 0.0002, 0.0003])).toBe(0.0006);
    expect(sum([1.234_56, 2.345_67, 3.456_78])).toBeCloseTo(7.037_01, 5);
  });
});
