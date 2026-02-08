import { lazyTake } from "@/Iterator/lazyTake";

describe("lazyTake", () => {
  it("takes the first n elements", () => {
    const result = [...lazyTake([1, 2, 3, 4, 5], 3)];
    expect(result).toEqual([1, 2, 3]);
  });

  it("takes all when n exceeds length", () => {
    const result = [...lazyTake([1, 2], 5)];
    expect(result).toEqual([1, 2]);
  });

  it("takes nothing when n is 0", () => {
    const result = [...lazyTake([1, 2, 3], 0)];
    expect(result).toEqual([]);
  });

  it("handles empty iterable", () => {
    const result = [...lazyTake([], 5)];
    expect(result).toEqual([]);
  });

  it("stops iterating after n elements (early return)", () => {
    let yieldCount = 0;
    function* source() {
      for (let i = 0; i < 100; i += 1) {
        yieldCount += 1;
        yield i;
      }
    }
    const result = [...lazyTake(source(), 3)];
    expect(result).toEqual([0, 1, 2]);
    expect(yieldCount).toBeLessThanOrEqual(4);
  });

  it("works with Set as iterable", () => {
    const set = new Set([10, 20, 30, 40, 50]);
    const result = [...lazyTake(set, 2)];
    expect(result).toEqual([10, 20]);
  });

  it("chains with other lazy operations", () => {
    function* naturals() {
      let n = 1;
      while (true) {
        yield n;
        n += 1;
      }
    }
    const result = [...lazyTake(naturals(), 5)];
    expect(result).toEqual([1, 2, 3, 4, 5]);
  });
});
