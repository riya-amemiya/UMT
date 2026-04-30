import { randomFloat } from "@/Random/randomFloat";

describe("randomFloat", () => {
  it("returns values within the half-open interval", () => {
    for (let index = 0; index < 100; index += 1) {
      const value = randomFloat(5, 10);
      expect(value).toBeGreaterThanOrEqual(5);
      expect(value).toBeLessThan(10);
    }
  });

  it("returns the lower bound when min equals max", () => {
    expect(randomFloat(7, 7)).toBe(7);
  });

  it("supports negative ranges", () => {
    for (let index = 0; index < 50; index += 1) {
      const value = randomFloat(-5, -1);
      expect(value).toBeGreaterThanOrEqual(-5);
      expect(value).toBeLessThan(-1);
    }
  });
});
