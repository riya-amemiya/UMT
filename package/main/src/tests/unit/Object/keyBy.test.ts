import { keyBy } from "@/Object/keyBy";

describe("keyBy", () => {
  it("プロパティ名でオブジェクトを生成できる", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "b2", name: "Bob" },
    ];

    const result = keyBy(input, "id");

    expect(result).toEqual({
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    });
  });

  it("カスタム関数でキーを生成できる", () => {
    const input = [
      { dir: "left", code: 97 },
      { dir: "right", code: 100 },
    ];

    const result = keyBy(input, (o) => String.fromCharCode(o.code));

    expect(result).toEqual({
      a: { dir: "left", code: 97 },
      d: { dir: "right", code: 100 },
    });
  });

  it("空の配列の場合は空のオブジェクトを返す", () => {
    const result = keyBy([], "id");
    expect(result).toEqual({});
  });

  it("重複するキーがある場合は後のものが優先される", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "a1", name: "Alex" },
    ];

    const result = keyBy(input, "id");

    expect(result).toEqual({
      a1: { id: "a1", name: "Alex" },
    });
  });

  it("オブジェクトを入力として受け付ける", () => {
    const input = {
      first: { id: "a1", name: "Alice" },
      second: { id: "b2", name: "Bob" },
    };

    const result = keyBy(input, "id");

    expect(result).toEqual({
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    });
  });
  it("iterateeが未指定の場合はidentity関数として動作する", () => {
    const input = ["a", "b"];
    const result = keyBy(input);

    expect(result).toEqual({
      a: "a",
      b: "b",
    });
  });
});
