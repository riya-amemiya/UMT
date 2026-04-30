import { debounceAsync } from "@/Async/debounceAsync";
import { sleep } from "@/Async/sleep";

describe("debounceAsync", () => {
  it("invokes the underlying function once for rapid calls and resolves all callers with the latest result", async () => {
    const function_ = jest.fn(async (value: number) => value * 2);
    const debounced = debounceAsync(function_, 10);
    const promises = [debounced(1), debounced(2), debounced(3)];
    const results = await Promise.all(promises);
    expect(function_).toHaveBeenCalledTimes(1);
    expect(function_).toHaveBeenCalledWith(3);
    expect(results).toEqual([6, 6, 6]);
  });

  it("invokes the underlying function again after the wait elapsed", async () => {
    const function_ = jest.fn(async (value: number) => value);
    const debounced = debounceAsync(function_, 5);
    await debounced(1);
    await sleep(10);
    await debounced(2);
    expect(function_).toHaveBeenCalledTimes(2);
  });

  it("rejects all pending callers when cancelled", async () => {
    const debounced = debounceAsync(async (value: number) => value, 50);
    const pending = debounced(1);
    debounced.cancel();
    await expect(pending).rejects.toThrow("debounceAsync cancelled");
  });

  it("propagates rejection from underlying function", async () => {
    const debounced = debounceAsync(() => Promise.reject(new Error("boom")), 5);
    await expect(debounced()).rejects.toThrow("boom");
  });

  it("cancel is a no-op when nothing is pending", () => {
    const debounced = debounceAsync(async () => 1, 10);
    expect(() => {
      debounced.cancel();
    }).not.toThrow();
  });
});
