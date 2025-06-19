import { timSort } from "@/Array/timSort";

describe("timSort", () => {
  it("should return an empty array when sorting an empty array", () => {
    expect(timSort([])).toEqual([]);
  });

  it("should return the same array when it's already sorted", () => {
    expect(timSort([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a reverse-sorted array", () => {
    expect(timSort([3, 2, 1])).toEqual([1, 2, 3]);
  });

  it("should correctly sort a random array", () => {
    const array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    const sortedArray = [...array].sort((a, b) => a - b);
    expect(timSort(array)).toEqual(sortedArray);
  });

  it("should correctly sort an array with duplicate elements", () => {
    expect(timSort([2, 3, 3, 1, 2])).toEqual([1, 2, 2, 3, 3]);
  });

  it("should correctly sort an array with large number of elements", () => {
    const largeArray = Array.from({ length: 10000 }, () =>
      Math.floor(Math.random() * 10000),
    );
    const sortedArray = [...largeArray].sort((a, b) => a - b);
    expect(timSort(largeArray)).toEqual(sortedArray);
  });

  it("should correctly sort an array with negative numbers", () => {
    expect(timSort([5, -1, 3, 2, 4, -5, 1, -2, 0])).toEqual([
      -5, -2, -1, 0, 1, 2, 3, 4, 5,
    ]);
  });

  it("should sort array in descending order", () => {
    expect(timSort([1, 2, 3], (a, b) => b - a)).toEqual([3, 2, 1]);
  });

  it("should sort a portion of the array", () => {
    expect(timSort([1, 3, 2, 5, 4], (a, b) => a - b, 1, 3)).toEqual([
      1, 2, 3, 5, 4,
    ]);
  });

  it("should maintain stability when sorting objects", () => {
    const array = [
      { value: 2, order: 1 },
      { value: 1, order: 2 },
      { value: 2, order: 3 },
      { value: 1, order: 4 },
    ];
    const result = timSort(array, (a, b) => a.value - b.value);
    expect(result.map((x) => x.order)).toEqual([2, 4, 1, 3]); // original order within same values
  });

  it("should handle arrays just larger than MIN_RUN", () => {
    const array = Array.from({ length: 33 }, (_, i) => 33 - i);
    const result = timSort(array);
    expect(result).toEqual(Array.from({ length: 33 }, (_, i) => i + 1));
  });

  it("should handle arrays with repeated elements near run boundaries", () => {
    // Creates runs with repeated elements at boundaries
    const array: number[] = [];
    for (let i = 0; i < 64; i++) {
      array.push(Math.floor(i / 16)); // Creates 4 values, each repeated 16 times
    }
    const shuffled = [...array].sort(() => Math.random() - 0.5);
    expect(timSort(shuffled)).toEqual(array);
  });

  it("should handle arrays with all identical elements", () => {
    const array = new Array(100).fill(1);
    expect(timSort(array)).toEqual(array);
  });

  it("should handle strings and numbers separately", () => {
    const strings = ["b", "a", "d", "c"];
    const numbers = [3, 1, 4, 2];
    expect(timSort(strings)).toEqual(["a", "b", "c", "d"]);
    expect(timSort(numbers)).toEqual([1, 2, 3, 4]);
  });

  it("should handle booleans", () => {
    const array = [true, false, true, false];
    expect(timSort(array)).toEqual([false, false, true, true]);
  });

  it("should handle null and undefined", () => {
    const array = [null, undefined, null, undefined];
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    const compareFunction = (a: any, b: any) => {
      if (a === b) {
        return 0;
      }
      if (a === null) {
        return -1;
      }
      if (b === null) {
        return 1;
      }
      if (a === undefined) {
        return -1;
      }
      if (b === undefined) {
        return 1;
      }
      return 0;
    };
    expect(timSort(array, compareFunction)).toEqual([
      null,
      null,
      undefined,
      undefined,
    ]);
  });

  it("should handle custom comparison for mixed types", () => {
    const array = ["b", 2, "a", 1];
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    const compareFunction = (a: any, b: any) => {
      const aStr = String(a);
      const bStr = String(b);
      return aStr.localeCompare(bStr);
    };
    expect(timSort(array, compareFunction)).toEqual([1, 2, "a", "b"]);
  });
});
