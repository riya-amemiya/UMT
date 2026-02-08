import { memoize } from "@/Function/memoize";

describe("memoize", () => {
  it("should cache and return the same result for the same argument", () => {
    const fn = jest.fn((n: number) => n * 2);
    const memoized = memoize(fn);

    expect(memoized(5)).toBe(10);
    expect(memoized(5)).toBe(10);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should compute for different arguments", () => {
    const fn = jest.fn((n: number) => n * 2);
    const memoized = memoize(fn);

    expect(memoized(5)).toBe(10);
    expect(memoized(10)).toBe(20);
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("should expose the cache", () => {
    const memoized = memoize((n: number) => n * 2);

    memoized(5);
    expect(memoized.cache.size).toBe(1);
    expect(memoized.cache.get(5)).toBe(10);
  });

  it("should evict the oldest entry when maxSize is exceeded", () => {
    const fn = jest.fn((n: number) => n * 2);
    const memoized = memoize(fn, { maxSize: 2 });

    memoized(1);
    memoized(2);
    memoized(3);

    expect(memoized.cache.size).toBe(2);
    expect(memoized.cache.has(1)).toBe(false);
    expect(memoized.cache.has(2)).toBe(true);
    expect(memoized.cache.has(3)).toBe(true);
  });

  it("should use a custom resolver for cache keys", () => {
    const fn = jest.fn((a: number, b: number) => a + b);
    const memoized = memoize(fn, {
      resolver: (a: unknown, b: unknown) => `${a}-${b}`,
    });

    expect(memoized(1, 2)).toBe(3);
    expect(memoized(1, 2)).toBe(3);
    expect(fn).toHaveBeenCalledTimes(1);

    expect(memoized(2, 1)).toBe(3);
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("should handle string keys by default", () => {
    const fn = jest.fn((s: string) => s.toUpperCase());
    const memoized = memoize(fn);

    expect(memoized("hello")).toBe("HELLO");
    expect(memoized("hello")).toBe("HELLO");
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should evict oldest entry with resolver and maxSize", () => {
    const fn = jest.fn((a: number, b: number) => a + b);
    const memoized = memoize(fn, {
      maxSize: 2,
      resolver: (a: unknown, b: unknown) => `${a}-${b}`,
    });

    memoized(1, 2);
    memoized(3, 4);
    memoized(5, 6);

    expect(memoized.cache.size).toBe(2);
    expect(memoized.cache.has("1-2")).toBe(false);
    expect(memoized.cache.has("3-4")).toBe(true);
    expect(memoized.cache.has("5-6")).toBe(true);
  });
});
