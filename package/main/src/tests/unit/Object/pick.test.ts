import { pick } from "@/Object/pick";

describe("pick関数のテスト", () => {
  test("単一のキーを選択", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: 1 });
  });

  test("複数のキーを選択", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("存在しないキーを選択", () => {
    const obj = { a: 1, b: 2 };
    // @ts-ignore
    const result = pick(obj, "c");
    expect(result).toEqual({});
  });

  test("空のオブジェクトから選択", () => {
    const obj = {};
    // @ts-ignore
    const result = pick(obj, "a");
    expect(result).toEqual({});
  });

  test("キーが空の場合", () => {
    const obj = { a: 1, b: 2 };
    const result = pick(obj);
    expect(result).toEqual({});
  });

  test("すべてのキーを選択", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual(obj);
  });

  test("重複したキーを選択", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("ネストされたオブジェクトから選択", () => {
    const obj = { a: { b: 1 }, c: 2 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: { b: 1 } });
  });

  test("nullまたはundefinedのプロパティを持つオブジェクトの選択", () => {
    const obj = { a: null, b: undefined, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual({ a: null, b: undefined, c: 3 });
  });

  test("配列を含むオブジェクトからの選択", () => {
    const obj = { a: [1, 2, 3], b: 4 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: [1, 2, 3] });
  });
});
