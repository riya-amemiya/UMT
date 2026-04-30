import { randomBoolean } from "@/Random/randomBoolean";

describe("randomBoolean", () => {
  it("returns a boolean", () => {
    expect(typeof randomBoolean()).toBe("boolean");
  });

  it("returns true when probability is 1", () => {
    for (let index = 0; index < 50; index += 1) {
      expect(randomBoolean(1)).toBe(true);
    }
  });

  it("returns false when probability is 0", () => {
    for (let index = 0; index < 50; index += 1) {
      expect(randomBoolean(0)).toBe(false);
    }
  });

  it("respects probability bias roughly", () => {
    let trueCount = 0;
    const trials = 2000;
    for (let index = 0; index < trials; index += 1) {
      if (randomBoolean(0.9)) {
        trueCount += 1;
      }
    }
    expect(trueCount / trials).toBeGreaterThan(0.7);
  });
});
