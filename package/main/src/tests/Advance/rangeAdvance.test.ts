import { rangeAdvance } from "@/Advance/rangeAdvance";

describe("rangeAdvance", () => {
  it("returns an array of numbers from 0 to start when only start is provided", () => {
    expect(rangeAdvance(5)).toEqual([0, 1, 2, 3, 4]);
  });

  it("returns an array of numbers from start to end when both start and end are provided", () => {
    expect(rangeAdvance(2, 5)).toEqual([2, 3, 4]);
  });

  it("returns an array of numbers that satisfy the conditional expression when a conditional expression is provided", () => {
    const isEven = (num: number) => num % 2 === 0;
    expect(rangeAdvance(0, 10, isEven)).toEqual([0, 2, 4, 6, 8, 10]);
  });

  it("returns an empty array when start is greater than end", () => {
    expect(rangeAdvance(5, 2)).toEqual([]);
  });

  it("returns an empty array when start is equal to end and the conditional expression is not satisfied", () => {
    const isOdd = (num: number) => num % 2 !== 0;
    expect(rangeAdvance(5, 5, isOdd)).toEqual([5]);
  });
});
