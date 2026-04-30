import { pSettled } from "@/Async/pSettled";

describe("pSettled", () => {
  it("resolves with mixed fulfilled and rejected results", async () => {
    const result = await pSettled([
      Promise.resolve(1),
      Promise.reject(new Error("nope")),
    ]);
    expect(result[0]).toEqual({ status: "fulfilled", value: 1 });
    expect(result[1].status).toBe("rejected");
  });

  it("returns empty array for empty input", async () => {
    expect(await pSettled([])).toEqual([]);
  });

  it("supports thunks (lazy promise factories)", async () => {
    let started = 0;
    const result = await pSettled(
      [
        (): Promise<number> => {
          started += 1;
          return Promise.resolve(1);
        },
        (): Promise<number> => {
          started += 1;
          return Promise.resolve(2);
        },
      ],
      1,
    );
    expect(started).toBe(2);
    expect(
      result.map((r) => (r.status === "fulfilled" ? r.value : null)),
    ).toEqual([1, 2]);
  });

  it("respects concurrency limit", async () => {
    let active = 0;
    let peak = 0;
    const makeTask = (index: number) => async (): Promise<number> => {
      active += 1;
      peak = Math.max(peak, active);
      await new Promise((resolve) => {
        setTimeout(resolve, 5);
      });
      active -= 1;
      return index;
    };
    const tasks = Array.from({ length: 6 }, (_, index) => makeTask(index));
    await pSettled(tasks, 2);
    expect(peak).toBeLessThanOrEqual(2);
  });

  it("preserves input order", async () => {
    const result = await pSettled([
      new Promise<number>((resolve) => {
        setTimeout(() => {
          resolve(1);
        }, 20);
      }),
      Promise.resolve(2),
    ]);
    expect(
      result.map((r) => (r.status === "fulfilled" ? r.value : null)),
    ).toEqual([1, 2]);
  });

  it("treats limit of 0 as unlimited", async () => {
    const result = await pSettled([Promise.resolve(1), Promise.resolve(2)], 0);
    expect(result).toHaveLength(2);
  });
});
