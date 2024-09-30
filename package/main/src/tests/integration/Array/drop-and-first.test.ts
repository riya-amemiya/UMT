import { first } from "@/Array/first";
import { drop } from "@/Array/drop";

describe("Integration test for 'drop' and 'first' functions", () => {
  it("should drop n elements from the array and return the first element", () => {
    expect(first(drop([1, 2, 3, 4, 5]))).toBe(2);
    expect(first(drop([1, 2, 3, 4, 5], 2))).toBe(3);
    expect(first(drop([1, 2, 3, 4, 5], 3))).toBe(4);
    expect(first(drop([1, 2, 3, 4, 5], 4))).toBe(5);
    expect(first(drop([1, 2, 3, 4, 5], 2, "right"))).toBe(1);
    expect(first(drop([1, 2, 3, 4, 5], 3, "left"))).toBe(4);
  });
});
