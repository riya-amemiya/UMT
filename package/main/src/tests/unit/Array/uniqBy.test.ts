import { uniqBy } from "@/Array/uniqBy";

describe("uniqBy", () => {
  test("should remove duplicates based on selector function", () => {
    const data = [
      { id: 1, name: "Alice" },
      { id: 2, name: "Bob" },
      { id: 1, name: "Alice Duplicate" },
      { id: 3, name: "Charlie" },
    ];
    expect(uniqBy(data, (item) => item.id)).toEqual([
      { id: 1, name: "Alice" },
      { id: 2, name: "Bob" },
      { id: 3, name: "Charlie" },
    ]);
  });

  test("should handle empty array", () => {
    expect(uniqBy([], (x) => x)).toEqual([]);
  });

  test("should work with string selector", () => {
    const data = ["apple", "banana", "apricot", "blueberry"];
    expect(uniqBy(data, (str) => str[0])).toEqual(["apple", "banana"]);
  });

  test("should work with number array", () => {
    const data = [1.1, 1.2, 2.1, 2.2, 3.1];
    expect(uniqBy(data, (num) => Math.floor(num))).toEqual([1.1, 2.1, 3.1]);
  });

  test("should preserve order of first occurrence", () => {
    const data = [
      { type: "fruit", name: "apple" },
      { type: "vegetable", name: "carrot" },
      { type: "fruit", name: "orange" },
      { type: "meat", name: "chicken" },
    ];
    expect(uniqBy(data, (item) => item.type)).toEqual([
      { type: "fruit", name: "apple" },
      { type: "vegetable", name: "carrot" },
      { type: "meat", name: "chicken" },
    ]);
  });

  test("should handle complex selector functions", () => {
    const data = [
      { x: 1, y: 2 },
      { x: 2, y: 1 },
      { x: 1, y: 3 },
      { x: 3, y: 1 },
    ];
    expect(uniqBy(data, (item) => item.x + item.y)).toEqual([
      { x: 1, y: 2 },
      { x: 1, y: 3 },
    ]);
  });

  test("should work with primitive values", () => {
    const data = [1, 2, 3, 11, 12, 13];
    expect(uniqBy(data, (num) => num % 10)).toEqual([1, 2, 3]);
  });

  test("should handle array with all same selector values", () => {
    const data = [1, 2, 3, 4];
    expect(uniqBy(data, () => "same")).toEqual([1]);
  });

  test("should handle selector returning different types", () => {
    const data = [
      { value: 1, flag: true },
      { value: 2, flag: false },
      { value: 3, flag: true },
      { value: 4, flag: false },
    ];
    expect(uniqBy(data, (item) => item.flag)).toEqual([
      { value: 1, flag: true },
      { value: 2, flag: false },
    ]);
  });
});
