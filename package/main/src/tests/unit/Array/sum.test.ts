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
});
