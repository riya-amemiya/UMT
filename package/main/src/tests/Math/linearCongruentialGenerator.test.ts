import { linearCongruentialGenerator } from "@/Math/linearCongruentialGenerator";

describe("linearCongruentialGenerator", () => {
  it("乱数を生成する", () => {
    const lcg = linearCongruentialGenerator(12345);
    const result = lcg();
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });

  it("同じシード値で同じ乱数列を生成する", () => {
    const lcg1 = linearCongruentialGenerator(12345);
    const lcg2 = linearCongruentialGenerator(12345);
    expect(lcg1()).toBe(lcg2());
    expect(lcg1()).toBe(lcg2());
    expect(lcg1()).toBe(lcg2());
  });

  it("異なるシード値で異なる乱数列を生成する", () => {
    const lcg1 = linearCongruentialGenerator(12345);
    const lcg2 = linearCongruentialGenerator(54321);
    expect(lcg1()).not.toBe(lcg2());
  });

  it("カスタムパラメータで乱数を生成する", () => {
    const lcg = linearCongruentialGenerator(12345, 1103515245, 12345, 2 ** 31);
    const result = lcg();
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });
});
