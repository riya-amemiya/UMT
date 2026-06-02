import { timeout } from "@/Async/timeout";

describe("timeout", () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.clearAllTimers();
    jest.useRealTimers();
  });

  it("resolves when promise completes before timeout", async () => {
    const promise = new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve("done");
      }, 10);
    });
    const result = timeout(promise, 1000);
    await jest.advanceTimersByTimeAsync(10);
    await expect(result).resolves.toBe("done");
  });

  it("rejects with timeout error when promise exceeds time", async () => {
    const promise = new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve("done");
      }, 10_000);
    });
    const result = timeout(promise, 50);
    const assertion = expect(result).rejects.toThrow("Timed out after 50ms");
    await jest.advanceTimersByTimeAsync(50);
    await assertion;
  });

  it("rejects with original error when promise fails", async () => {
    const promise = new Promise<string>((_, reject) => {
      setTimeout(() => {
        reject(new Error("original error"));
      }, 10);
    });
    const result = timeout(promise, 1000);
    const assertion = expect(result).rejects.toThrow("original error");
    await jest.advanceTimersByTimeAsync(10);
    await assertion;
  });

  it("clears timeout after resolution", async () => {
    const clearTimeoutSpy = jest.spyOn(global, "clearTimeout");
    const promise = Promise.resolve(42);
    await timeout(promise, 1000);
    expect(clearTimeoutSpy).toHaveBeenCalled();
    clearTimeoutSpy.mockRestore();
  });

  it("clears timeout after rejection", async () => {
    const clearTimeoutSpy = jest.spyOn(global, "clearTimeout");
    const promise = Promise.reject(new Error("fail"));
    await expect(timeout(promise, 1000)).rejects.toThrow("fail");
    expect(clearTimeoutSpy).toHaveBeenCalled();
    clearTimeoutSpy.mockRestore();
  });
});
