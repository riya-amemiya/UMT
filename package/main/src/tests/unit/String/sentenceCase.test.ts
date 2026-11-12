import { sentenceCase } from "@/String/sentenceCase";

describe("sentenceCase", () => {
  it("should convert according to JSDoc examples", () => {
    expect(sentenceCase("helloWorld foo-bar")).toBe("Hello world foo bar");
    expect(sentenceCase("XMLHttpRequest")).toBe("Xml http request");
    expect(sentenceCase("")).toBe("");
  });

  it("should lower case every word after the first", () => {
    expect(sentenceCase("HELLO WORLD")).toBe("Hello world");
    expect(sentenceCase("snake_case_value")).toBe("Snake case value");
  });

  it("should handle a single word", () => {
    expect(sentenceCase("hello")).toBe("Hello");
  });

  it("should return an empty string when there are no words", () => {
    expect(sentenceCase("  --  ")).toBe("");
  });

  it("should handle non-ASCII letters", () => {
    expect(sentenceCase("\u{c9}CLAIR au CHOCOLAT")).toBe(
      "\u{c9}clair au chocolat",
    );
  });
});
