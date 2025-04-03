import { repeatedTrial } from "@/Math/repeatedTrial";

describe("repeatedTrial", () => {
  it("should calculate basic probability cases", () => {
    expect(repeatedTrial(4, 2, { x: 1, y: 3 })).toEqual([8, 27]);
    expect(repeatedTrial(5, 2, { x: 1, y: 2 })).toEqual([5, 16]);
    expect(repeatedTrial(3, 2, { x: 1, y: 2 })).toEqual([3, 8]);
  });

  it("should handle extreme probability cases", () => {
    expect(repeatedTrial(3, 3, { x: 2, y: 3 })).toEqual([8, 27]);
    expect(repeatedTrial(4, 4, { x: 1, y: 2 })).toEqual([1, 16]);
  });

  it("should handle probability with simple fractions", () => {
    expect(repeatedTrial(2, 1, { x: 1, y: 2 })).toEqual([1, 2]);
    expect(repeatedTrial(3, 2, { x: 1, y: 3 })).toEqual([2, 9]);
  });
});
