import { trimStartCharacters } from "@/String/trimStartCharacters";

describe("trimStartCharacters", () => {
  it("指定された文字を文字列の先頭から取り除く", () => {
    expect(trimStartCharacters("---Hello World---", "-")).toBe(
      "Hello World---",
    );
  });

  it("文字列に指定された文字が含まれていない場合、変更を加えずにそのまま返す", () => {
    expect(trimStartCharacters("Hello World", "-")).toBe("Hello World");
  });

  it("空の文字列が与えられた場合、空の文字列を返す", () => {
    expect(trimStartCharacters("", "-")).toBe("");
  });

  it("すべての文字が取り除かれる場合、空の文字列を返す", () => {
    expect(trimStartCharacters("---", "-")).toBe("");
  });

  it("複数の異なる文字を取り除く", () => {
    expect(trimStartCharacters("abcHello World", "abc")).toBe("Hello World");
  });
});
