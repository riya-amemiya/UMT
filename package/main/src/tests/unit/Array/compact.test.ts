import { compact } from "@/Array/compact";
import _ from "lodash";

describe("compact関数のテスト", () => {
  test("真偽値と数値が混在する配列", () => {
    expect(compact([0, 1, false, 2, "", 3])).toEqual([1, 2, 3]);
    expect(_.compact([0, 1, false, 2, "", 3])).toEqual([1, 2, 3]);
  });

  test("全て真の値が含まれる配列", () => {
    expect(compact([1, 2, 3, "a", true])).toEqual([1, 2, 3, "a", true]);
    expect(_.compact([1, 2, 3, "a", true])).toEqual([1, 2, 3, "a", true]);
  });

  test("全て偽の値が含まれる配列", () => {
    expect(compact([0, false, "", null, undefined, NaN])).toEqual([]);
    expect(_.compact([0, false, "", null, undefined, NaN])).toEqual([]);
  });

  test("空の配列", () => {
    expect(compact([])).toEqual([]);
    expect(_.compact([])).toEqual([]);
  });

  test("配列にnullやundefinedが含まれる", () => {
    expect(compact([null, undefined, "hello"])).toEqual(["hello"]);
    expect(_.compact([null, undefined, "hello"])).toEqual(["hello"]);
  });

  test("配列に0や''などの他の偽の値が含まれる", () => {
    expect(compact([0, 1, "", 2, " ", 3])).toEqual([1, 2, " ", 3]);
    expect(_.compact([0, 1, "", 2, " ", 3])).toEqual([1, 2, " ", 3]);
  });

  test("配列にオブジェクトや配列などの真値が含まれる", () => {
    expect(compact([{}, [], 0, false, "text"])).toEqual([{}, [], "text"]);
    expect(_.compact([{}, [], 0, false, "text"])).toEqual([{}, [], "text"]);
  });

  test("配列にNaNが含まれる", () => {
    expect(compact([NaN, 1, 2, NaN])).toEqual([1, 2]);
    expect(_.compact([NaN, 1, 2, NaN])).toEqual([1, 2]);
  });

  test("多種多様なデータ型が含まれる", () => {
    expect(
      compact([0, 1, "a", "", null, undefined, [], {}, false, true, NaN]),
    ).toEqual([1, "a", [], {}, true]);
    expect(
      _.compact([0, 1, "a", "", null, undefined, [], {}, false, true, NaN]),
    ).toEqual([1, "a", [], {}, true]);
  });

  test("配列が変更されずに新しい配列を返すこと", () => {
    const original = [0, 1, false, 2];
    const result = compact(original);
    expect(result).toEqual([1, 2]);
    expect(original).toEqual([0, 1, false, 2]);

    const lodashOriginal = [0, 1, false, 2];
    const lodashResult = _.compact(lodashOriginal);
    expect(lodashResult).toEqual([1, 2]);
    expect(lodashOriginal).toEqual([0, 1, false, 2]);
  });
});
