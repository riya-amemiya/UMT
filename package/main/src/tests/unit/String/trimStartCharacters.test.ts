import { trimStartCharacters } from "@/String/trimStartCharacters";

describe("trimStartCharacters", () => {
  it("should remove specified characters from the start of string", () => {
    expect(trimStartCharacters("---Hello", "-")).toBe("Hello");
    expect(trimStartCharacters("!!!world", "!")).toBe("world");
  });

  it("should handle multiple different characters to trim", () => {
    expect(trimStartCharacters("...123test", ".123")).toBe("test");
    expect(trimStartCharacters("---...text", ".-")).toBe("text");
  });

  it("should return original string if no characters match", () => {
    expect(trimStartCharacters("hello", "x")).toBe("hello");
    expect(trimStartCharacters("test", "xyz")).toBe("test");
  });

  it("should handle empty input string", () => {
    expect(trimStartCharacters("", "x")).toBe("");
    expect(trimStartCharacters("", "")).toBe("");
  });

  it("should return empty string if all characters are trimmed", () => {
    expect(trimStartCharacters("xxxxx", "x")).toBe("");
    expect(trimStartCharacters(".....", ".")).toBe("");
  });

  it("should return original string if trim characters is empty", () => {
    expect(trimStartCharacters("hello", "")).toBe("hello");
    expect(trimStartCharacters("123test", "")).toBe("123test");
  });

  it("should handle non-ascii characters", () => {
    expect(trimStartCharacters("。。。こんにちは", "。")).toBe("こんにちは");
    expect(trimStartCharacters("！！Hello", "！")).toBe("Hello");
  });
});
