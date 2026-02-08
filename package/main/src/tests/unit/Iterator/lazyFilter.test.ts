import { lazyFilter } from "@/Iterator/lazyFilter";

describe("lazyFilter", () => {
  it("filters values lazily", () => {
    const result = [...lazyFilter([1, 2, 3, 4, 5], (n) => n % 2 === 0)];
    expect(result).toEqual([2, 4]);
  });

  it("provides index to the predicate", () => {
    const result = [...lazyFilter([10, 20, 30, 40], (_, i) => i >= 2)];
    expect(result).toEqual([30, 40]);
  });

  it("handles empty iterable", () => {
    const result = [...lazyFilter([], (n: number) => n > 0)];
    expect(result).toEqual([]);
  });

  it("handles no matching elements", () => {
    const result = [...lazyFilter([1, 2, 3], () => false)];
    expect(result).toEqual([]);
  });

  it("handles all matching elements", () => {
    const result = [...lazyFilter([1, 2, 3], () => true)];
    expect(result).toEqual([1, 2, 3]);
  });

  it("evaluates lazily", () => {
    let callCount = 0;
    const gen = lazyFilter([1, 2, 3, 4, 5], (n) => {
      callCount += 1;
      return n > 3;
    });

    expect(callCount).toBe(0);
    gen.next();
    expect(callCount).toBeGreaterThan(0);
  });

  it("works with Set as iterable", () => {
    const set = new Set([1, 2, 3, 4, 5]);
    const result = [...lazyFilter(set, (n) => n > 3)];
    expect(result).toEqual([4, 5]);
  });
});
