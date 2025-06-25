import { chunk } from "@/Array/chunk";
import _ from "lodash";

describe("chunkArray function", () => {
  it("splits an array into chunks of size n", () => {
    const input = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    const expected = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles empty arrays", () => {
    const input: number[] = [];
    const n = 3;
    const expected: number[][] = [];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles n larger than array length", () => {
    const input = [0, 1, 2];
    const n = 5;
    const expected = [[0, 1, 2]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("type checks", () => {
    const input: [0, 1, 2, 3, 4, 5, 6, 7] = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    const output = chunk(input, n);
    const expected: typeof output = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7],
    ];
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("does not mutate the input array", () => {
    const input = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    chunk(input, n);
    expect(input).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
    _.chunk(input, n);
    expect(input).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
  });

  it("handles chunk size of 1", () => {
    const input = [1, 2, 3, 4, 5];
    const n = 1;
    const expected = [[1], [2], [3], [4], [5]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with string elements", () => {
    const input = ["a", "b", "c", "d", "e", "f", "g"];
    const n = 3;
    const expected = [["a", "b", "c"], ["d", "e", "f"], ["g"]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with mixed types", () => {
    const input = [1, "a", true, null, undefined, 2, "b", false];
    const n = 3;
    const expected = [
      [1, "a", true],
      [null, undefined, 2],
      ["b", false],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with objects", () => {
    const obj1 = { id: 1 };
    const obj2 = { id: 2 };
    const obj3 = { id: 3 };
    const obj4 = { id: 4 };
    const input = [obj1, obj2, obj3, obj4];
    const n = 2;
    const expected = [
      [obj1, obj2],
      [obj3, obj4],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with special numeric values", () => {
    const input = [
      Number.NaN,
      Number.POSITIVE_INFINITY,
      Number.NEGATIVE_INFINITY,
      -0,
      0,
      1,
    ];
    const n = 2;
    const expected = [
      [Number.NaN, Number.POSITIVE_INFINITY],
      [Number.NEGATIVE_INFINITY, -0],
      [0, 1],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with symbols", () => {
    const sym1 = Symbol("test1");
    const sym2 = Symbol("test2");
    const sym3 = Symbol.for("shared");
    const input = [sym1, sym2, sym3];
    const n = 2;
    const expected = [[sym1, sym2], [sym3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles large arrays efficiently", () => {
    const input = Array.from({ length: 10_000 }, (_, i) => i);
    const n = 1000;
    const output = chunk(input, n);
    expect(output).toHaveLength(10);
    expect(output[0]).toEqual(Array.from({ length: 1000 }, (_, i) => i));
    expect(output[9]).toEqual(Array.from({ length: 1000 }, (_, i) => i + 9000));

    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(output);
  });

  it("handles arrays with Date objects", () => {
    const date1 = new Date("2024-01-01");
    const date2 = new Date("2024-01-02");
    const date3 = new Date("2024-01-03");
    const input = [date1, date2, date3];
    const n = 2;
    const expected = [[date1, date2], [date3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with functions", () => {
    const fn1 = () => 1;
    const fn2 = () => 2;
    const fn3 = () => 3;
    const input = [fn1, fn2, fn3];
    const n = 2;
    const expected = [[fn1, fn2], [fn3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles nested arrays", () => {
    const nestedArray1 = [1, 2];
    const nestedArray2 = [3, 4];
    const nestedArray3 = [5, 6];
    const nestedArray4 = [7, 8];
    const input = [nestedArray1, nestedArray2, nestedArray3, nestedArray4];
    const n = 3;
    const expected = [
      [nestedArray1, nestedArray2, nestedArray3],
      [nestedArray4],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with regular expressions", () => {
    const regex1 = /test1/g;
    const regex2 = /test2/i;
    const regex3 = /test3/;
    const input = [regex1, regex2, regex3];
    const n = 2;
    const expected = [[regex1, regex2], [regex3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with Map and Set objects", () => {
    const map1 = new Map([["key1", "value1"]]);
    const set1 = new Set([1, 2, 3]);
    const map2 = new Map([["key2", "value2"]]);
    const input = [map1, set1, map2];
    const n = 2;
    const expected = [[map1, set1], [map2]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with BigInt values", () => {
    const bigInt1 = BigInt(123);
    const bigInt2 = BigInt(456);
    const bigInt3 = BigInt(789);
    const input = [bigInt1, bigInt2, bigInt3];
    const n = 2;
    const expected = [[bigInt1, bigInt2], [bigInt3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles arrays with class instances", () => {
    class TestClass {
      constructor(public value: number) {}
    }

    const instance1 = new TestClass(1);
    const instance2 = new TestClass(2);
    const instance3 = new TestClass(3);
    const input = [instance1, instance2, instance3];
    const n = 2;
    const expected = [[instance1, instance2], [instance3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles perfect division (no remainder)", () => {
    const input = [1, 2, 3, 4, 5, 6];
    const n = 3;
    const expected = [
      [1, 2, 3],
      [4, 5, 6],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles array equal to chunk size", () => {
    const input = [1, 2, 3];
    const n = 3;
    const expected = [[1, 2, 3]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles single element array", () => {
    const input = [42];
    const n = 1;
    const expected = [[42]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles TypedArrays", () => {
    const uint8Array = new Uint8Array([1, 2, 3, 4, 5]);
    const int32Array = new Int32Array([10, 20, 30]);
    const input = [uint8Array, int32Array];
    const n = 1;
    const expected = [[uint8Array], [int32Array]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("maintains reference equality for objects", () => {
    const obj = { shared: true };
    const input = [obj, obj, obj];
    const n = 2;
    const output = chunk(input, n);
    expect(output).toEqual([[obj, obj], [obj]]);
    expect(output[0][0]).toBe(obj);
    expect(output[0][1]).toBe(obj);
    expect(output[1][0]).toBe(obj);
  });
});
