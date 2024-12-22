import { pop } from "@/Array/pop";

describe("pop", () => {
  it("should remove and return the last element of the array", () => {
    const array = [1, 2, 3];
    const result = pop(array);
    expect(result).toBe(3);
    expect(array).toEqual([1, 2, 3]);
  });

  it("should return undefined for an empty array", () => {
    const array: number[] = [];
    const result = pop(array);
    expect(result).toBeUndefined();
  });

  it("should handle arrays with one element", () => {
    const array = [42];
    const result = pop(array);
    expect(result).toBe(42);
    expect(array).toEqual([42]);
  });

  it("should handle arrays with multiple types", () => {
    const array = [1, "two", true];
    const result = pop(array);
    expect(result).toBe(true);
    expect(array).toEqual([1, "two", true]);
  });

  it("should handle arrays with nested arrays", () => {
    const array = [1, [2, 3], 4];
    const result = pop(array);
    expect(result).toBe(4);
    expect(array).toEqual([1, [2, 3], 4]);
  });

  it("should handle arrays with objects", () => {
    const array = [{ a: 1 }, { b: 2 }];
    const result = pop(array);
    expect(result).toEqual({ b: 2 });
    expect(array).toEqual([{ a: 1 }, { b: 2 }]);
  });
});
