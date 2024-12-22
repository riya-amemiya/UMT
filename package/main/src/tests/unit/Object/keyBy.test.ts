import { keyBy } from "@/Object/keyBy";
import _ from "lodash";

describe("keyBy", () => {
  it("プロパティ名でオブジェクトを生成できる", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "b2", name: "Bob" },
    ];
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("カスタム関数でキーを生成できる", () => {
    const input = [
      { dir: "left", code: 97 },
      { dir: "right", code: 100 },
    ];

    const result = {
      a: { dir: "left", code: 97 },
      d: { dir: "right", code: 100 },
    };
    const output = keyBy(input, (o) => String.fromCharCode(o.code));
    const lodashOutput = _.keyBy(input, (o) => String.fromCharCode(o.code));

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("空の配列の場合は空のオブジェクトを返す", () => {
    const result = {};
    const output = keyBy([], "id");
    const lodashOutput = _.keyBy([], "id");
    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("重複するキーがある場合は後のものが優先される", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "a1", name: "Alex" },
    ];
    const result = {
      a1: { id: "a1", name: "Alex" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("オブジェクトを入力として受け付ける", () => {
    const input = {
      first: { id: "a1", name: "Alice" },
      second: { id: "b2", name: "Bob" },
    };
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });
  it("iterateeが未指定の場合はidentity関数として動作する", () => {
    const input = ["a", "b"];
    const result = {
      a: "a",
      b: "b",
    };
    const output = keyBy(input);
    const lodashOutput = _.keyBy(input);

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });
});
