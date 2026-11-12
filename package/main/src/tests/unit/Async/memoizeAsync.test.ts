import { defer } from "@/Async/defer";
import { memoizeAsync } from "@/Async/memoizeAsync";

describe("memoizeAsync", () => {
  it("caches resolved values keyed by the first argument", async () => {
    const function_ = jest.fn((n: number) => Promise.resolve(n * 2));
    const memoized = memoizeAsync(function_);
    await expect(memoized(5)).resolves.toBe(10);
    await expect(memoized(5)).resolves.toBe(10);
    await expect(memoized(6)).resolves.toBe(12);
    expect(function_).toHaveBeenCalledTimes(2);
    expect(memoized.cache.get(5)).toBe(10);
    expect(memoized.cache.size).toBe(2);
  });

  it("shares one in-flight promise between concurrent calls with the same key", async () => {
    const deferred = defer<string>();
    const function_ = jest.fn((_key: string) => deferred.promise);
    const memoized = memoizeAsync(function_);
    const first = memoized("a");
    const second = memoized("a");
    expect(second).toBe(first);
    expect(memoized.cache.size).toBe(0);
    deferred.resolve("done");
    await expect(Promise.all([first, second])).resolves.toEqual([
      "done",
      "done",
    ]);
    expect(function_).toHaveBeenCalledTimes(1);
    expect(memoized.cache.get("a")).toBe("done");
  });

  it("does not cache rejected calls", async () => {
    const function_ = jest
      .fn<Promise<number>, [number]>()
      .mockRejectedValueOnce(new Error("fail"))
      .mockResolvedValueOnce(42);
    const memoized = memoizeAsync(function_);
    const first = memoized(1);
    const second = memoized(1);
    await expect(first).rejects.toThrow("fail");
    await expect(second).rejects.toThrow("fail");
    expect(memoized.cache.has(1)).toBe(false);
    await expect(memoized(1)).resolves.toBe(42);
    expect(function_).toHaveBeenCalledTimes(2);
  });

  it("invokes the function again when retried right after a rejection", async () => {
    const function_ = jest
      .fn<Promise<string>, [string]>()
      .mockRejectedValueOnce(new Error("fail"))
      .mockResolvedValueOnce("ok");
    const memoized = memoizeAsync(function_);
    const retried = await memoized("k").catch(() => memoized("k"));
    expect(retried).toBe("ok");
    expect(function_).toHaveBeenCalledTimes(2);
  });

  it("rejects and does not cache when the function throws synchronously", async () => {
    let calls = 0;
    const memoized = memoizeAsync((n: number): Promise<number> => {
      calls += 1;
      if (calls === 1) {
        throw new Error("sync");
      }
      return Promise.resolve(n);
    });
    await expect(memoized(3)).rejects.toThrow("sync");
    await expect(memoized(3)).resolves.toBe(3);
    expect(calls).toBe(2);
  });

  it("caches resolved undefined values", async () => {
    const function_ = jest.fn((_key: string) => Promise.resolve(undefined));
    const memoized = memoizeAsync(function_);
    await memoized("x");
    await expect(memoized("x")).resolves.toBeUndefined();
    expect(function_).toHaveBeenCalledTimes(1);
  });

  it("uses a custom resolver for the cache key", async () => {
    const function_ = jest.fn((a: number, b: number) => Promise.resolve(a + b));
    const memoized = memoizeAsync(function_, {
      resolver: (a, b) => `${a},${b}`,
    });
    await expect(memoized(1, 2)).resolves.toBe(3);
    await expect(memoized(1, 2)).resolves.toBe(3);
    await expect(memoized(1, 3)).resolves.toBe(4);
    expect(function_).toHaveBeenCalledTimes(2);
    expect([...memoized.cache.keys()]).toEqual(["1,2", "1,3"]);
  });

  it("evicts the oldest entry when maxSize is reached", async () => {
    const function_ = jest.fn((n: number) => Promise.resolve(n));
    const memoized = memoizeAsync(function_, { maxSize: 2 });
    await memoized(1);
    await memoized(2);
    await memoized(3);
    expect([...memoized.cache.keys()]).toEqual([2, 3]);
    await memoized(1);
    expect(function_).toHaveBeenCalledTimes(4);
  });

  it("preserves the this context", async () => {
    const context = {
      factor: 3,
      run: memoizeAsync(function (this: { factor: number }, n: number) {
        return Promise.resolve(n * this.factor);
      }),
    };
    await expect(context.run(2)).resolves.toBe(6);
  });
});
