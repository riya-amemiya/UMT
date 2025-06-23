import { trimEndCharacters } from "@/String/trimEndCharacters";

describe("trimEndCharacters", () => {
  it("should remove specified characters from the end of string", () => {
    expect(trimEndCharacters("hellooo", "o")).toBe("hell");
    expect(trimEndCharacters("banana!!!", "!")).toBe("banana");
  });

  it("should handle multiple different characters to trim", () => {
    expect(trimEndCharacters("hello123...", "123.")).toBe("hello");
    expect(trimEndCharacters("test---...", ".-")).toBe("test");
  });

  it("should return original string if no characters match", () => {
    expect(trimEndCharacters("apple", "x")).toBe("apple");
    expect(trimEndCharacters("test", "xyz")).toBe("test");
  });

  it("should handle empty input string", () => {
    expect(trimEndCharacters("", "x")).toBe("");
    expect(trimEndCharacters("", "")).toBe("");
  });

  it("should return empty string if all characters are trimmed", () => {
    expect(trimEndCharacters("xxxxx", "x")).toBe("");
    expect(trimEndCharacters("....", ".")).toBe("");
  });

  it("should return original string if trim characters is empty", () => {
    expect(trimEndCharacters("hello", "")).toBe("hello");
    expect(trimEndCharacters("test123", "")).toBe("test123");
  });

  it("should handle non-ascii characters", () => {
    expect(trimEndCharacters("こんにちは。。。", "。")).toBe("こんにちは");
    expect(trimEndCharacters("Hello！！", "！")).toBe("Hello");
  });
});
