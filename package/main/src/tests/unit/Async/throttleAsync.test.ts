import { sleep } from "@/Async/sleep";
import { throttleAsync } from "@/Async/throttleAsync";

describe("throttleAsync", () => {
  it("returns the same in-flight promise to concurrent callers", async () => {
    const function_ = jest.fn(async (value: number) => {
      await sleep(10);
      return value;
    });
    const throttled = throttleAsync(function_, 0);
    const a = throttled(1);
    const b = throttled(2);
    expect(a).toBe(b);
    await Promise.all([a, b]);
    expect(function_).toHaveBeenCalledTimes(1);
  });

  it("rejects calls within the lockout window after completion", async () => {
    const function_ = jest.fn(async (value: number) => value);
    const throttled = throttleAsync(function_, 1000);
    await throttled(1);
    await expect(throttled(2)).rejects.toThrow(
      "throttleAsync window not elapsed",
    );
    expect(function_).toHaveBeenCalledTimes(1);
  });

  it("allows a new invocation after the wait window", async () => {
    const function_ = jest.fn(async (value: number) => value);
    const throttled = throttleAsync(function_, 5);
    await throttled(1);
    await sleep(15);
    await throttled(2);
    expect(function_).toHaveBeenCalledTimes(2);
  });

  it("cancel clears the lockout allowing a new invocation immediately", async () => {
    const function_ = jest.fn(async (value: number) => value);
    const throttled = throttleAsync(function_, 1000);
    await throttled(1);
    throttled.cancel();
    await throttled(2);
    expect(function_).toHaveBeenCalledTimes(2);
  });

  it("propagates rejection", async () => {
    const throttled = throttleAsync(() => Promise.reject(new Error("boom")), 5);
    await expect(throttled()).rejects.toThrow("boom");
  });

  it("does not clear a newer in-flight when an older promise resolves after cancel", async () => {
    let resolveFirst: ((value: number) => void) | undefined;
    let resolveSecond: ((value: number) => void) | undefined;
    const function_ = jest.fn((value: number) => {
      if (value === 1) {
        return new Promise<number>((resolve) => {
          resolveFirst = resolve;
        });
      }
      return new Promise<number>((resolve) => {
        resolveSecond = resolve;
      });
    });
    const throttled = throttleAsync(function_, 1000);
    const first = throttled(1);
    throttled.cancel();
    const second = throttled(2);
    resolveFirst?.(1);
    await first;
    resolveSecond?.(2);
    await second;
    expect(function_).toHaveBeenCalledTimes(2);
  });
});
