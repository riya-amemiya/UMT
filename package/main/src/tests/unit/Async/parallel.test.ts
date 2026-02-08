import { parallel } from "@/Async/parallel";

describe("parallel", () => {
  it("processes all items and returns results in order", async () => {
    const items = [1, 2, 3, 4, 5];
    const results = await parallel(2, items, async (n) => n * 10);
    expect(results).toEqual([10, 20, 30, 40, 50]);
  });

  it("respects the concurrency limit", async () => {
    let running = 0;
    let maxRunning = 0;

    const items = [1, 2, 3, 4, 5, 6];
    await parallel(3, items, async (n) => {
      running += 1;
      if (running > maxRunning) {
        maxRunning = running;
      }
      await new Promise<void>((resolve) => {
        setTimeout(resolve, 10);
      });
      running -= 1;
      return n;
    });

    expect(maxRunning).toBeLessThanOrEqual(3);
  });

  it("handles empty array", async () => {
    const results = await parallel(2, [], async (n: number) => n * 2);
    expect(results).toEqual([]);
  });

  it("handles limit larger than items length", async () => {
    const items = [1, 2];
    const results = await parallel(10, items, async (n) => n + 1);
    expect(results).toEqual([2, 3]);
  });

  it("rejects when any task fails", async () => {
    const items = [1, 2, 3];
    await expect(
      parallel(2, items, (n) => {
        if (n === 2) {
          return Promise.reject(new Error("fail"));
        }
        return Promise.resolve(n);
      }),
    ).rejects.toThrow("fail");
  });

  it("provides index to the function", async () => {
    const items = ["a", "b", "c"];
    const results = await parallel(
      2,
      items,
      async (item, index) => `${item}-${index}`,
    );
    expect(results).toEqual(["a-0", "b-1", "c-2"]);
  });

  it("handles limit of 1 (sequential)", async () => {
    const order: number[] = [];
    const items = [1, 2, 3];
    await parallel(1, items, (n) => {
      order.push(n);
      return Promise.resolve(n);
    });
    expect(order).toEqual([1, 2, 3]);
  });
});
