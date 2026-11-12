import { retry } from "@/Async/retry";

describe("retry", () => {
  it("returns the result on first success", async () => {
    const function_ = jest.fn(() => Promise.resolve("ok"));
    await expect(retry(function_)).resolves.toBe("ok");
    expect(function_).toHaveBeenCalledTimes(1);
  });

  it("retries on failure and eventually resolves", async () => {
    let attempts = 0;
    const function_ = jest.fn(() => {
      attempts += 1;
      if (attempts < 3) {
        return Promise.reject(new Error("nope"));
      }
      return Promise.resolve("ok");
    });
    await expect(retry(function_, { retries: 3, delay: 1 })).resolves.toBe(
      "ok",
    );
    expect(function_).toHaveBeenCalledTimes(3);
  });

  it("rejects after exhausting retries", async () => {
    const function_ = jest.fn(() => Promise.reject(new Error("nope")));
    await expect(retry(function_, { retries: 2, delay: 1 })).rejects.toThrow(
      "nope",
    );
    expect(function_).toHaveBeenCalledTimes(3);
  });

  it("respects shouldRetry returning false", async () => {
    const function_ = jest.fn(() => Promise.reject(new Error("fatal")));
    await expect(
      retry(function_, {
        retries: 5,
        delay: 1,
        shouldRetry: () => false,
      }),
    ).rejects.toThrow("fatal");
    expect(function_).toHaveBeenCalledTimes(1);
  });

  it("invokes onRetry callback before each retry", async () => {
    let attempts = 0;
    const onRetry = jest.fn();
    await retry(
      () => {
        attempts += 1;
        if (attempts < 2) {
          return Promise.reject(new Error("nope"));
        }
        return Promise.resolve("ok");
      },
      { retries: 2, delay: 1, onRetry },
    );
    expect(onRetry).toHaveBeenCalledTimes(1);
  });

  it("supports linear backoff", async () => {
    let attempts = 0;
    await retry(
      () => {
        attempts += 1;
        if (attempts < 2) {
          return Promise.reject(new Error("nope"));
        }
        return Promise.resolve("ok");
      },
      { retries: 2, delay: 1, backoff: "linear" },
    );
    expect(attempts).toBe(2);
  });

  it("supports exponential backoff", async () => {
    let attempts = 0;
    await retry(
      () => {
        attempts += 1;
        if (attempts < 2) {
          return Promise.reject(new Error("nope"));
        }
        return Promise.resolve("ok");
      },
      { retries: 2, delay: 1, backoff: "exponential" },
    );
    expect(attempts).toBe(2);
  });

  it("supports jitter", async () => {
    let attempts = 0;
    await retry(
      () => {
        attempts += 1;
        if (attempts < 2) {
          return Promise.reject(new Error("nope"));
        }
        return Promise.resolve("ok");
      },
      { retries: 2, delay: 1, jitter: true },
    );
    expect(attempts).toBe(2);
  });

  it("caps each backoff delay at maxDelay", async () => {
    const timeoutSpy = jest.spyOn(globalThis, "setTimeout");
    const function_ = jest.fn(() => Promise.reject(new Error("nope")));
    await expect(
      retry(function_, {
        retries: 3,
        delay: 1000,
        backoff: "exponential",
        maxDelay: 2,
      }),
    ).rejects.toThrow("nope");
    expect(timeoutSpy.mock.calls.map((call) => call[1])).toEqual([2, 2, 2]);
    timeoutSpy.mockRestore();
  });

  it("keeps delays below maxDelay unchanged", async () => {
    const timeoutSpy = jest.spyOn(globalThis, "setTimeout");
    const function_ = jest.fn(() => Promise.reject(new Error("nope")));
    await expect(
      retry(function_, {
        retries: 2,
        delay: 1,
        backoff: "linear",
        maxDelay: 100,
      }),
    ).rejects.toThrow("nope");
    expect(timeoutSpy.mock.calls.map((call) => call[1])).toEqual([1, 2]);
    timeoutSpy.mockRestore();
  });

  it("applies maxDelay after jitter", async () => {
    const timeoutSpy = jest.spyOn(globalThis, "setTimeout");
    const randomSpy = jest.spyOn(Math, "random").mockReturnValue(0.5);
    const function_ = jest.fn(() => Promise.reject(new Error("nope")));
    await expect(
      retry(function_, {
        retries: 2,
        delay: 8,
        jitter: true,
        maxDelay: 3,
      }),
    ).rejects.toThrow("nope");
    expect(timeoutSpy.mock.calls.map((call) => call[1])).toEqual([3, 3]);
    randomSpy.mockRestore();
    timeoutSpy.mockRestore();
  });

  it("rejects immediately when signal is already aborted", async () => {
    const controller = new AbortController();
    controller.abort(new Error("cancelled"));
    await expect(
      retry(() => Promise.resolve("ok"), { signal: controller.signal }),
    ).rejects.toThrow("cancelled");
  });

  it("falls back to a generic Aborted error when signal has no reason", async () => {
    const controller = new AbortController();
    const originalAbort = controller.abort.bind(controller);
    Object.defineProperty(controller.signal, "reason", { value: undefined });
    Object.defineProperty(controller.signal, "aborted", { value: true });
    await expect(
      retry(() => Promise.resolve("ok"), { signal: controller.signal }),
    ).rejects.toThrow("Aborted");
    expect(originalAbort).toBeDefined();
  });
});
