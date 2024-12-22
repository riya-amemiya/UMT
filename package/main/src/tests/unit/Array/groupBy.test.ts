import { groupBy } from "@/Array/groupBy";

describe("groupBy", () => {
  it("数値に基づいて奇数と偶数にグループ化する", () => {
    const array = [1, 2, 3, 4, 5];
    const result = groupBy(array, (num) => (num % 2 === 0 ? "even" : "odd"));
    expect(result).toEqual({ odd: [1, 3, 5], even: [2, 4] });
  });

  it("文字列の長さに基づいてグループ化する", () => {
    const array = ["one", "two", "three", "four", "five"];
    const result = groupBy(array, (str) => str.length.toString());
    expect(result).toEqual({
      "3": ["one", "two"],
      "4": ["four", "five"],
      "5": ["three"],
    });
  });

  it("真偽値に基づいてグループ化する", () => {
    const array = [true, false, true, false, true];
    const result = groupBy(array, (bool) => bool.toString());
    expect(result).toEqual({ true: [true, true, true], false: [false, false] });
  });

  it("オブジェクトのプロパティに基づいてグループ化する", () => {
    const array: {
      type: "fruit" | "vegetable";
      name: string;
    }[] = [
      { type: "fruit", name: "apple" },
      { type: "vegetable", name: "carrot" },
      { type: "fruit", name: "banana" },
      { type: "vegetable", name: "lettuce" },
    ];
    const result = groupBy(array, (item) => item.type);
    expect(result).toEqual({
      fruit: [
        { type: "fruit", name: "apple" },
        { type: "fruit", name: "banana" },
      ],
      vegetable: [
        { type: "vegetable", name: "carrot" },
        { type: "vegetable", name: "lettuce" },
      ],
    });
  });

  it("undefinedを返す関数を渡した場合、すべての要素がundefinedキーにグループ化される", () => {
    const array = [1, 2, 3, 4, 5];
    // @ts-ignore
    const result = groupBy(array, () => undefined);
    expect(result).toEqual({ undefined: [1, 2, 3, 4, 5] });
  });

  it("空の配列を渡した場合、空のオブジェクトが返される", () => {
    const array: number[] = [];
    const result = groupBy(array, (num) => (num % 2 === 0 ? "even" : "odd"));
    expect(result).toEqual({});
  });
});
