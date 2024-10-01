import { trimEndCharacters } from "@/String/trimEndCharacters";

describe("trimEndCharacters", () => {
  it("末尾の特定の文字を取り除く", () => {
    expect(trimEndCharacters("hellooo", "o")).toBe("hell");
  });

  it("末尾の複数の異なる文字を取り除く", () => {
    expect(trimEndCharacters("banana!!!", "!")).toBe("banana");
  });

  it("文字列に含まれない文字を取り除こうとしても変化しない", () => {
    expect(trimEndCharacters("apple", "x")).toBe("apple");
  });

  it("空の文字列を渡した場合、空の文字列を返す", () => {
    expect(trimEndCharacters("", "x")).toBe("");
  });

  it("すべての文字が取り除かれる場合、空の文字列を返す", () => {
    expect(trimEndCharacters("xxxxx", "x")).toBe("");
  });

  it("第二引数に空の文字列を渡した場合、元の文字列をそのまま返す", () => {
    expect(trimEndCharacters("hello", "")).toBe("hello");
  });
});
