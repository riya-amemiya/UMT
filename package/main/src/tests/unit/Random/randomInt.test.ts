import { randomInt } from "@/Random/randomInt";

describe("randomInt", () => {
  it("returns integers within the closed interval", () => {
    for (let index = 0; index < 200; index += 1) {
      const value = randomInt(1, 6);
      expect(Number.isInteger(value)).toBe(true);
      expect(value).toBeGreaterThanOrEqual(1);
      expect(value).toBeLessThanOrEqual(6);
    }
  });

  it("returns the bound when min equals max", () => {
    expect(randomInt(3, 3)).toBe(3);
  });

  it("supports negative ranges", () => {
    for (let index = 0; index < 50; index += 1) {
      const value = randomInt(-3, 3);
      expect(value).toBeGreaterThanOrEqual(-3);
      expect(value).toBeLessThanOrEqual(3);
    }
  });

  it("ceils the lower bound and floors the upper bound", () => {
    for (let index = 0; index < 50; index += 1) {
      const value = randomInt(0.4, 5.7);
      expect(value).toBeGreaterThanOrEqual(1);
      expect(value).toBeLessThanOrEqual(5);
    }
  });
});
