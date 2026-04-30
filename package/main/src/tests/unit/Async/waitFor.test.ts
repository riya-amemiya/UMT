import { waitFor } from "@/Async/waitFor";

describe("waitFor", () => {
  it("resolves immediately when condition is initially truthy", async () => {
    await expect(waitFor(() => 42)).resolves.toBe(42);
  });

  it("resolves once the condition becomes truthy", async () => {
    let counter = 0;
    const result = await waitFor(
      () => {
        counter += 1;
        return counter >= 3 ? "done" : null;
      },
      { interval: 1, timeout: 500 },
    );
    expect(result).toBe("done");
  });

  it("rejects on timeout", async () => {
    await expect(
      waitFor(() => null, { interval: 5, timeout: 20 }),
    ).rejects.toThrow(/timed out/);
  });

  it("supports async predicate", async () => {
    let counter = 0;
    const result = await waitFor(
      () => {
        counter += 1;
        return Promise.resolve(counter >= 2 ? counter : 0);
      },
      { interval: 1, timeout: 500 },
    );
    expect(result).toBe(2);
  });

  it("rejects when signal is already aborted", async () => {
    const controller = new AbortController();
    controller.abort(new Error("nope"));
    await expect(
      waitFor(() => 1, { signal: controller.signal }),
    ).rejects.toThrow("nope");
  });

  it("rejects when condition throws", async () => {
    await expect(
      waitFor(
        () => {
          throw new Error("boom");
        },
        { timeout: 100 },
      ),
    ).rejects.toThrow("boom");
  });

  it("aborts during polling", async () => {
    const controller = new AbortController();
    setTimeout(() => {
      controller.abort(new Error("stopped"));
    }, 5);
    await expect(
      waitFor(() => null, {
        signal: controller.signal,
        interval: 2,
        timeout: 1000,
      }),
    ).rejects.toThrow("stopped");
  });

  it("aborts post-condition without an explicit reason", async () => {
    const controller = new AbortController();
    setTimeout(() => {
      controller.abort();
    }, 5);
    await expect(
      waitFor(() => null, {
        signal: controller.signal,
        interval: 50,
        timeout: 1000,
      }),
    ).rejects.toThrow();
  });

  it("falls back to a generic Aborted error when signal has no reason on entry", async () => {
    const controller = new AbortController();
    Object.defineProperty(controller.signal, "reason", { value: undefined });
    Object.defineProperty(controller.signal, "aborted", { value: true });
    await expect(
      waitFor(() => null, { signal: controller.signal }),
    ).rejects.toThrow("Aborted");
  });
});
