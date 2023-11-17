import { trimCharacters } from "@/String/trimCharacters";

describe("trimCharacters", () => {
  it("指定された文字を文字列の前後から削除する", () => {
    expect(trimCharacters("---Hello World---", "-")).toBe("Hello World");
  });

  it("文字列の中間にある指定された文字は削除しない", () => {
    expect(trimCharacters("---Hello-World---", "-")).toBe("Hello-World");
  });

  it("空の文字列を渡した場合、空の文字列を返す", () => {
    expect(trimCharacters("", "-")).toBe("");
  });

  it("削除する文字が指定されていない場合、元の文字列をそのまま返す", () => {
    expect(trimCharacters("Hello World", "")).toBe("Hello World");
  });
});
