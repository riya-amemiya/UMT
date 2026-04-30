import { randomChoice } from "@/Random/randomChoice";

describe("randomChoice", () => {
  it("returns an element from the array", () => {
    const items = ["a", "b", "c"] as const;
    for (let index = 0; index < 50; index += 1) {
      expect(items).toContain(randomChoice(items));
    }
  });

  it("returns the only element for single-item arrays", () => {
    expect(randomChoice([42])).toBe(42);
  });
});
