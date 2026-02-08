import { timeout } from "@/Async/timeout";

describe("timeout", () => {
  it("resolves when promise completes before timeout", async () => {
    const promise = new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve("done");
      }, 10);
    });
    const result = await timeout(promise, 1000);
    expect(result).toBe("done");
  });

  it("rejects with timeout error when promise exceeds time", async () => {
    const promise = new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve("done");
      }, 10_000);
    });
    await expect(timeout(promise, 50)).rejects.toThrow("Timed out after 50ms");
  });

  it("rejects with original error when promise fails", async () => {
    const promise = new Promise<string>((_, reject) => {
      setTimeout(() => {
        reject(new Error("original error"));
      }, 10);
    });
    await expect(timeout(promise, 1000)).rejects.toThrow("original error");
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
