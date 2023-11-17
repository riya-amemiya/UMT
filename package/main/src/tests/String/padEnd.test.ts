import { padEnd } from "@/String/padEnd";

describe("padEnd function", () => {
  test("文字列の末尾にパディングを追加する", () => {
    expect(padEnd("abc", 5, " ")).toBe("abc  ");
    expect(padEnd("hello", 10, "!")).toBe("hello!!!!!");
  });

  test("元の文字列が既に目標の長さ以上の場合、変更を加えない", () => {
    expect(padEnd("abc", 3, " ")).toBe("abc");
    expect(padEnd("longstring", 5, "!")).toBe("longstring");
  });

  test("パディング文字列が複数文字からなる場合", () => {
    expect(padEnd("abc", 10, "de")).toBe("abcdededed");
  });

  test("目標の長さが元の文字列の長さより短い場合は、元の文字列をそのまま返す", () => {
    expect(padEnd("abc", 2, " ")).toBe("abc");
  });

  test("パディング文字列が空の場合は、元の文字列をそのまま返す", () => {
    expect(padEnd("abc", 5, "")).toBe("abc");
  });
});
