import { linearCongruentialGenerator } from "@/Math/linearCongruentialGenerator";

describe("linearCongruentialGenerator", () => {
  it("should generate random numbers", () => {
    const result = linearCongruentialGenerator(8, 13, 3, 5);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBe(3);
  });

  it("should generate the same sequence for the same seed", () => {
    const result1 = linearCongruentialGenerator(12345);
    const result2 = linearCongruentialGenerator(12345);
    expect(result1).toBe(result2);
  });

  it("should generate different sequences for different seeds", () => {
    const result1 = linearCongruentialGenerator(12345);
    const result2 = linearCongruentialGenerator(54321);
    expect(result1).not.toBe(result2);
  });

  it("should generate numbers with custom parameters", () => {
    const result1 = linearCongruentialGenerator(
      12345,
      2 ** 31,
      1103515245,
      12345,
    );
    expect(result1).toBeGreaterThanOrEqual(0);
    expect(result1).toBe(1406932606);
  });
});
