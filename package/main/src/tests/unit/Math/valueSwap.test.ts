import { valueSwap } from "@/Math/valueSwap";
describe("valueSwap function", () => {
  it("should keep order when x < y", () => {
    expect(valueSwap(1, 2)).toEqual([1, 2]);
    expect(valueSwap(-2, -1)).toEqual([-2, -1]);
    expect(valueSwap(0.1, 0.2)).toEqual([0.1, 0.2]);
  });

  it("should swap order when x > y", () => {
    expect(valueSwap(2, 1)).toEqual([1, 2]);
    expect(valueSwap(-1, -2)).toEqual([-2, -1]);
    expect(valueSwap(0.2, 0.1)).toEqual([0.1, 0.2]);
  });

  it("should handle equal numbers", () => {
    expect(valueSwap(1, 1)).toEqual([1, 1]);
    expect(valueSwap(0, 0)).toEqual([0, 0]);
    expect(valueSwap(-1, -1)).toEqual([-1, -1]);
  });

  it("should handle decimal numbers", () => {
    expect(valueSwap(1.5, 1.2)).toEqual([1.2, 1.5]);
    expect(valueSwap(0.5, 1.5)).toEqual([0.5, 1.5]);
  });

  it("should handle negative numbers", () => {
    expect(valueSwap(-5, 5)).toEqual([-5, 5]);
    expect(valueSwap(5, -5)).toEqual([-5, 5]);
  });
});
