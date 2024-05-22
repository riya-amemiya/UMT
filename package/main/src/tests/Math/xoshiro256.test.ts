import { xoshiro256 } from "@/Math/xoshiro256";

describe("xoshiro256", () => {
  it("乱数を生成する", () => {
    const xoshiro = xoshiro256([1, 2, 3, 4]);
    const result = xoshiro();
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });

  it("同じシード値で同じ乱数列を生成する", () => {
    const xoshiro1 = xoshiro256([1, 2, 3, 4]);
    const xoshiro2 = xoshiro256([1, 2, 3, 4]);
    expect(xoshiro1()).toBe(xoshiro2());
    expect(xoshiro1()).toBe(xoshiro2());
    expect(xoshiro1()).toBe(xoshiro2());
  });

  it("異なるシード値で異なる乱数列を生成する", () => {
    const xoshiro1 = xoshiro256([1, 2, 3, 4]);
    const xoshiro2 = xoshiro256([4, 3, 2, 1]);
    expect(xoshiro1()).not.toBe(xoshiro2());
  });
});
