import { mapSeries } from "@/Async/mapSeries";

describe("mapSeries", () => {
  it("processes all items and returns results in order", async () => {
    const results = await mapSeries([1, 2, 3, 4, 5], async (n) => n * 10);
    expect(results).toEqual([10, 20, 30, 40, 50]);
  });

  it("runs tasks strictly sequentially", async () => {
    let running = 0;
    let maxRunning = 0;
    const order: number[] = [];

    await mapSeries([1, 2, 3, 4], async (n) => {
      running += 1;
      if (running > maxRunning) {
        maxRunning = running;
      }
      order.push(n);
      await new Promise<void>((resolve) => {
        setTimeout(resolve, 5);
      });
      running -= 1;
      return n;
    });

    expect(maxRunning).toBe(1);
    expect(order).toEqual([1, 2, 3, 4]);
  });

  it("handles empty array", async () => {
    const results = await mapSeries([], async (n: number) => n * 2);
    expect(results).toEqual([]);
  });

  it("provides index to the function", async () => {
    const results = await mapSeries(
      ["a", "b", "c"],
      async (item, index) => `${item}-${index}`,
    );
    expect(results).toEqual(["a-0", "b-1", "c-2"]);
  });

  it("rejects when any task fails", async () => {
    await expect(
      mapSeries([1, 2, 3], (n) => {
        if (n === 2) {
          return Promise.reject(new Error("fail"));
        }
        return Promise.resolve(n);
      }),
    ).rejects.toThrow("fail");
  });

  it("handles single element array", async () => {
    const results = await mapSeries([7], async (n) => n + 1);
    expect(results).toEqual([8]);
  });
});
