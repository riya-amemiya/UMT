import { xoshiro256 } from "@/Math/xoshiro256";

describe("xoshiro256", () => {
  it("乱数を生成する", () => {
    const result = xoshiro256([1, 2, 3, 4]);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });

  it("同じシード値で同じ乱数列を生成する", () => {
    const result1 = xoshiro256([1, 2, 3, 4]);
    const result2 = xoshiro256([1, 2, 3, 4]);
    expect(result1).toBe(result2);
    expect(result1).toBe(result2);
    expect(result1).toBe(result2);
  });

  it("異なるシード値で異なる乱数列を生成する", () => {
    const result1 = xoshiro256([1, 2, 3, 4]);
    const result2 = xoshiro256([4, 3, 2, 1]);
    expect(result1).not.toBe(result2);
  });
});
