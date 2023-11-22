import { quickSortAsync } from "@/Async/Array/quickSortAsync";

describe("quickSortAsync", () => {
  it("should sort an array of numbers", async () => {
    const array = [5, 3, 1, 4, 2];
    const result = await quickSortAsync(array);
    expect(result).toEqual([1, 2, 3, 4, 5]);
  });

  it("should sort an array of strings", async () => {
    const array = ["e", "c", "a", "d", "b"];
    const result = await quickSortAsync(array);
    expect(result).toEqual(["a", "b", "c", "d", "e"]);
  });

  it("should sort a large array", async () => {
    const array = Array.from({ length: 1000 }, (_, i) => 1000 - i);
    const result = await quickSortAsync(array);
    expect(result).toEqual(Array.from({ length: 1000 }, (_, i) => i + 1));
  });

  it("should sort an array of numbers in descending order", async () => {
    const array = [5, 3, 1, 4, 2];
    const result = await quickSortAsync(array, (a, b) => b - a);
    expect(result).toEqual([5, 4, 3, 2, 1]);
  });
});
