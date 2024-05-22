import { linearCongruentialGenerator } from "@/Math/linearCongruentialGenerator";

describe("linearCongruentialGenerator", () => {
  it("乱数を生成する", () => {
    const result = linearCongruentialGenerator(12345);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThan(1);
  });

  it("同じシード値で同じ乱数列を生成する", () => {
    const result1 = linearCongruentialGenerator(12345);
    const result2 = linearCongruentialGenerator(12345);
    expect(result1).toBe(result2);
    expect(result1).toBe(result2);
    expect(result1).toBe(result2);
  });

  it("異なるシード値で異なる乱数列を生成する", () => {
    const result1 = linearCongruentialGenerator(12345);
    const result2 = linearCongruentialGenerator(54321);
    expect(result1).not.toBe(result2);
  });

  it("カスタムパラメータで乱数を生成する", () => {
    const result1 = linearCongruentialGenerator(
      12345,
      1103515245,
      12345,
      2 ** 31,
    );
    expect(result1).toBeGreaterThanOrEqual(0);
    expect(result1).toBeLessThan(1);
  });
});
