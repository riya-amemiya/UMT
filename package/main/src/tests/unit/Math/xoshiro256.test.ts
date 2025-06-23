import { xoshiro256 } from "@/Math/xoshiro256";

describe("xoshiro256 function", () => {
  it("should generate random numbers within the default range", () => {
    const state: [number, number, number, number] = [1, 2, 3, 4];
    const result = xoshiro256(state);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });

  it("should generate the same sequence with the same seed values", () => {
    const state1: [number, number, number, number] = [1, 2, 3, 4];
    const state2: [number, number, number, number] = [1, 2, 3, 4];
    const result1 = xoshiro256(state1);
    const result2 = xoshiro256(state2);
    expect(result1).toBe(result2);
  });

  it("should generate different sequences with different seed values", () => {
    const state1: [number, number, number, number] = [1, 2, 3, 4];
    const state2: [number, number, number, number] = [4, 3, 2, 1];
    const result1 = xoshiro256(state1);
    const result2 = xoshiro256(state2);
    expect(result1).not.toBe(result2);
  });

  it("should respect the min and max parameters", () => {
    const state: [number, number, number, number] = [1, 2, 3, 4];
    const min = 10;
    const max = 20;
    const result = xoshiro256(state, min, max);
    expect(result).toBeGreaterThanOrEqual(min);
    expect(result).toBeLessThan(max);
  });
});
