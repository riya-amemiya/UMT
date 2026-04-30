import { seededRandom } from "@/Random/seededRandom";

describe("seededRandom", () => {
  it("produces values in [0, 1)", () => {
    const rand = seededRandom(42);
    for (let index = 0; index < 100; index += 1) {
      const value = rand();
      expect(value).toBeGreaterThanOrEqual(0);
      expect(value).toBeLessThan(1);
    }
  });

  it("is deterministic for the same numeric seed", () => {
    const a = seededRandom(123);
    const b = seededRandom(123);
    for (let index = 0; index < 20; index += 1) {
      expect(a()).toBe(b());
    }
  });

  it("is deterministic for the same string seed", () => {
    const a = seededRandom("hello");
    const b = seededRandom("hello");
    for (let index = 0; index < 20; index += 1) {
      expect(a()).toBe(b());
    }
  });

  it("produces different streams for different seeds", () => {
    const a = seededRandom(1);
    const b = seededRandom(2);
    let differs = false;
    for (let index = 0; index < 10; index += 1) {
      if (a() !== b()) {
        differs = true;
        break;
      }
    }
    expect(differs).toBe(true);
  });

  it("handles a numeric seed of 0", () => {
    const rand = seededRandom(0);
    for (let index = 0; index < 20; index += 1) {
      const value = rand();
      expect(value).toBeGreaterThanOrEqual(0);
      expect(value).toBeLessThan(1);
    }
  });

  it("handles an empty string seed", () => {
    const rand = seededRandom("");
    const value = rand();
    expect(value).toBeGreaterThanOrEqual(0);
    expect(value).toBeLessThan(1);
  });
});
