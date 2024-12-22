import { pickDeep } from "@/Object/pickDeep";

describe("pickDeep関数のテスト", () => {
  test("単純なキーの選択", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("ネストされたキーの選択", () => {
    const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
    const result = pickDeep(obj, "a.b.c", "f");
    expect(result).toEqual({ a: { b: { c: 1 } }, f: 4 });
  });

  test("存在しないキーの選択", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: 1 });
  });

  test("キーが空の場合", () => {
    const obj = { a: 1, b: 2 };
    const result = pickDeep(obj);
    expect(result).toEqual({});
  });

  test("配列を含むオブジェクトからの選択", () => {
    const obj = { a: [{ b: 1 }, { c: 2 }], d: 3 };
    const result = pickDeep(obj, "a", "d");
    expect(result).toEqual(obj);
  });

  test("nullまたはundefinedのプロパティを持つオブジェクトの選択", () => {
    const obj = { a: null, b: undefined, c: 3 };
    const result = pickDeep(obj, "a", "b", "c");
    expect(result).toEqual({ a: null, b: undefined, c: 3 });
  });

  test("ネストが深いキーの選択", () => {
    const obj = {
      a: {
        b: {
          c: {
            d: {
              e: 5,
            },
          },
        },
      },
    };
    const result = pickDeep(obj, "a.b.c.d.e");
    expect(result).toEqual({
      a: {
        b: {
          c: {
            d: {
              e: 5,
            },
          },
        },
      },
    });
  });

  test("選択キーがオブジェクト全体を指す場合", () => {
    const obj = { a: { b: 1, c: 2 }, d: 3 };
    const result = pickDeep(obj, "a");
    expect(result).toEqual({ a: { b: 1, c: 2 } });
  });

  test("キーがドットを含まない場合", () => {
    const obj = { a: { b: 1 }, c: 2 };
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: { b: 1 }, c: 2 });
  });

  test("選択キーが空文字列の場合", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pickDeep(obj, "");
    expect(result).toEqual({});
  });

  test("キーに数値が含まれる場合", () => {
    const obj = { a: { "1": 1, "2": 2 }, b: 3 };
    const result = pickDeep(obj, "a.1");
    expect(result).toEqual({ a: { "1": 1 } });
  });

  test("キーが存在しない深さ", () => {
    const obj = { a: { b: 1 } };
    // @ts-expect-error
    const result = pickDeep(obj, "a.b.c");
    expect(result).toEqual({ a: { b: {} } });
  });

  test("複数のキーを同時に選択する", () => {
    const obj = {
      user: { name: "Alice", address: { city: "Tokyo", zip: "12345" } },
      posts: [
        { id: 1, title: "Post 1" },
        { id: 2, title: "Post 2" },
      ],
    };
    const result = pickDeep(obj, "user.name", "user.address.city", "posts");
    expect(result).toEqual({
      user: { name: "Alice", address: { city: "Tokyo" } },
      posts: [
        { id: 1, title: "Post 1" },
        { id: 2, title: "Post 2" },
      ],
    });
  });

  test("選択キーが重複している場合", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pickDeep(obj, "a", "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });
});
