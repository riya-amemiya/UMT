import { first } from "@/Array/first";

describe("first", () => {
  it("should return the first element of a non-empty array", () => {
    expect(first([1, 2, 3])).toBe(1);
    expect(first(["a", "b", "c"])).toBe("a");
    expect(first([true, false, true])).toBe(true);
    expect(first([{ key: "value" }, { key: "another" }])).toEqual({
      key: "value",
    });
  });

  it("should return undefined for an empty array", () => {
    expect(first([])).toBeUndefined();
  });

  it("should handle arrays with different types of elements", () => {
    expect(first([1, "a", true])).toBe(1);
    expect(first([undefined, null, 0])).toBeUndefined();
  });

  it("should handle arrays with only one element", () => {
    expect(first([42])).toBe(42);
    expect(first(["single"])).toBe("single");
  });

  it("should handle arrays with special numeric values", () => {
    expect(first([Number.NaN, 1, 2])).toBe(Number.NaN);
    expect(first([Number.POSITIVE_INFINITY, 1, 2])).toBe(
      Number.POSITIVE_INFINITY,
    );
    expect(first([Number.NEGATIVE_INFINITY, 1, 2])).toBe(
      Number.NEGATIVE_INFINITY,
    );
    expect(first([-0, 1, 2])).toBe(-0);
    expect(first([0, 1, 2])).toBe(0);
  });

  it("should handle arrays with null and undefined values", () => {
    expect(first([null, 1, 2])).toBeNull();
    expect(first([undefined, 1, 2])).toBeUndefined();
    expect(first([null, null, null])).toBeNull();
    expect(first([undefined, undefined, undefined])).toBeUndefined();
  });

  it("should handle arrays with falsy values", () => {
    expect(first([false, true])).toBe(false);
    expect(first([0, 1, 2])).toBe(0);
    expect(first(["", "hello"])).toBe("");
    expect(first([null, "value"])).toBeNull();
    expect(first([undefined, "value"])).toBeUndefined();
  });

  it("should handle arrays with objects", () => {
    const obj1 = { id: 1, name: "first" };
    const obj2 = { id: 2, name: "second" };
    expect(first([obj1, obj2])).toBe(obj1);
    expect(first([obj1, obj2])).toEqual({ id: 1, name: "first" });
  });

  it("should handle arrays with functions", () => {
    const fn1 = () => "first";
    const fn2 = () => "second";
    expect(first([fn1, fn2])).toBe(fn1);
    const firstFn = first([fn1, fn2]);
    expect(firstFn).toBe(fn1);
    if (firstFn) {
      expect(firstFn()).toBe("first");
    }
  });

  it("should handle arrays with symbols", () => {
    const sym1 = Symbol("first");
    const sym2 = Symbol("second");
    const sym3 = Symbol.for("shared");
    expect(first([sym1, sym2, sym3])).toBe(sym1);
  });

  it("should handle arrays with Date objects", () => {
    const date1 = new Date("2024-01-01");
    const date2 = new Date("2024-01-02");
    expect(first([date1, date2])).toBe(date1);
    const firstDate = first([date1, date2]);
    expect(firstDate).toBe(date1);
    if (firstDate) {
      expect(firstDate.getTime()).toBe(date1.getTime());
    }
  });

  it("should handle arrays with regular expressions", () => {
    const regex1 = /first/g;
    const regex2 = /second/i;
    expect(first([regex1, regex2])).toBe(regex1);
    const firstRegex = first([regex1, regex2]);
    expect(firstRegex).toBe(regex1);
    if (firstRegex) {
      expect(firstRegex.source).toBe("first");
    }
  });

  it("should handle nested arrays", () => {
    const nestedArray1 = [1, 2, 3];
    const nestedArray2 = [4, 5, 6];
    expect(first([nestedArray1, nestedArray2])).toBe(nestedArray1);
    expect(first([nestedArray1, nestedArray2])).toEqual([1, 2, 3]);
  });

  it("should handle large arrays efficiently", () => {
    const largeArray = Array.from({ length: 10_000 }, (_, i) => i);
    expect(first(largeArray)).toBe(0);

    const largeStringArray = Array.from(
      { length: 10_000 },
      (_, i) => `item${i}`,
    );
    expect(first(largeStringArray)).toBe("item0");
  });

  it("should handle arrays with Map and Set objects", () => {
    const map1 = new Map([["key", "value"]]);
    const set1 = new Set([1, 2, 3]);
    expect(first([map1, set1])).toBe(map1);
    expect(first([set1, map1])).toBe(set1);
  });

  it("should handle arrays with class instances", () => {
    class TestClass {
      constructor(public value: number) {}
    }

    const instance1 = new TestClass(1);
    const instance2 = new TestClass(2);
    expect(first([instance1, instance2])).toBe(instance1);
    const firstInstance = first([instance1, instance2]);
    expect(firstInstance).toBe(instance1);
    if (firstInstance) {
      expect(firstInstance.value).toBe(1);
    }
  });

  it("should handle arrays with BigInt values", () => {
    const bigInt1 = BigInt(123);
    const bigInt2 = BigInt(456);
    expect(first([bigInt1, bigInt2])).toBe(bigInt1);
    expect(first([bigInt1, bigInt2])).toBe(123n);
  });

  it("should handle sparse arrays", () => {
    const sparseArray = [1, undefined, undefined, 4];
    expect(first(sparseArray)).toBe(1);

    const startWithEmpty = [undefined, undefined, 3, 4];
    expect(first(startWithEmpty)).toBeUndefined();
  });

  it("should preserve type information", () => {
    const numberArray = [1, 2, 3];
    const stringArray = ["a", "b", "c"];
    const mixedArray = [1, "a", true];

    expect(first(numberArray)).toBe(1);
    expect(first(stringArray)).toBe("a");
    expect(first(mixedArray)).toBe(1);
  });

  it("should handle ArrayBuffer and TypedArrays", () => {
    const uint8Array = new Uint8Array([1, 2, 3]);
    const int32Array = new Int32Array([10, 20, 30]);

    expect(first([uint8Array, int32Array])).toBe(uint8Array);
    expect(first([int32Array, uint8Array])).toBe(int32Array);
  });
});
