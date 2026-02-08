import { lazyMap } from "@/Iterator/lazyMap";

describe("lazyMap", () => {
  it("maps values lazily", () => {
    const result = [...lazyMap([1, 2, 3], (n) => n * 2)];
    expect(result).toEqual([2, 4, 6]);
  });

  it("provides index to the mapping function", () => {
    const result = [...lazyMap(["a", "b", "c"], (v, i) => `${v}-${i}`)];
    expect(result).toEqual(["a-0", "b-1", "c-2"]);
  });

  it("handles empty iterable", () => {
    const result = [...lazyMap([], (n: number) => n)];
    expect(result).toEqual([]);
  });

  it("evaluates lazily", () => {
    let callCount = 0;
    const gen = lazyMap([1, 2, 3, 4, 5], (n) => {
      callCount += 1;
      return n * 2;
    });

    expect(callCount).toBe(0);

    gen.next();
    expect(callCount).toBe(1);

    gen.next();
    expect(callCount).toBe(2);
  });

  it("works with Set as iterable", () => {
    const set = new Set([10, 20, 30]);
    const result = [...lazyMap(set, (n) => n + 1)];
    expect(result).toEqual([11, 21, 31]);
  });

  it("works with string as iterable", () => {
    const result = [...lazyMap("abc", (c) => c.toUpperCase())];
    expect(result).toEqual(["A", "B", "C"]);
  });

  it("works with generator as iterable", () => {
    function* source() {
      yield 1;
      yield 2;
      yield 3;
    }
    const result = [...lazyMap(source(), (n) => n * 10)];
    expect(result).toEqual([10, 20, 30]);
  });
});
