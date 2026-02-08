import { defer } from "@/Async/defer";

describe("defer", () => {
  it("resolves with the given value", async () => {
    const d = defer<number>();
    d.resolve(42);
    const result = await d.promise;
    expect(result).toBe(42);
  });

  it("rejects with the given reason", async () => {
    const d = defer<number>();
    d.reject(new Error("deferred error"));
    await expect(d.promise).rejects.toThrow("deferred error");
  });

  it("exposes resolve and reject as functions", () => {
    const d = defer<string>();
    expect(typeof d.resolve).toBe("function");
    expect(typeof d.reject).toBe("function");
    expect(d.promise).toBeInstanceOf(Promise);
  });

  it("works with async/await pattern", async () => {
    const d = defer<string>();
    setTimeout(() => {
      d.resolve("delayed");
    }, 10);
    const result = await d.promise;
    expect(result).toBe("delayed");
  });

  it("handles promise-like values", async () => {
    const d = defer<number>();
    d.resolve(Promise.resolve(99));
    const result = await d.promise;
    expect(result).toBe(99);
  });
});
