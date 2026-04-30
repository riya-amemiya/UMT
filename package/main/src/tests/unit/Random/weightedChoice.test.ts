import { weightedChoice } from "@/Random/weightedChoice";

describe("weightedChoice", () => {
  it("returns one of the weighted values", () => {
    const items = [
      { value: "a", weight: 1 },
      { value: "b", weight: 1 },
    ];
    const allowed = new Set(["a", "b"]);
    for (let index = 0; index < 50; index += 1) {
      expect(allowed.has(weightedChoice(items))).toBe(true);
    }
  });

  it("favors heavier weights statistically", () => {
    const items = [
      { value: "a", weight: 1 },
      { value: "b", weight: 9 },
    ];
    let bCount = 0;
    const trials = 2000;
    for (let index = 0; index < trials; index += 1) {
      if (weightedChoice(items) === "b") {
        bCount += 1;
      }
    }
    expect(bCount / trials).toBeGreaterThan(0.8);
  });

  it("ignores non-positive weights", () => {
    const items = [
      { value: "skip", weight: 0 },
      { value: "winner", weight: 5 },
    ];
    for (let index = 0; index < 50; index += 1) {
      expect(weightedChoice(items)).toBe("winner");
    }
  });

  it("treats negative weights as ignored", () => {
    const items = [
      { value: "a", weight: -3 },
      { value: "b", weight: 5 },
    ];
    for (let index = 0; index < 50; index += 1) {
      expect(weightedChoice(items)).toBe("b");
    }
  });
});
