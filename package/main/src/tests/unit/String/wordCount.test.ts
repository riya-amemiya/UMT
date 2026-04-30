import { wordCount } from "@/String/wordCount";

describe("wordCount", () => {
  it("counts space-separated words", () => {
    expect(wordCount("hello world")).toBe(2);
  });

  it("counts camelCase boundaries", () => {
    expect(wordCount("camelCaseSplit")).toBe(3);
  });

  it("returns zero for empty string", () => {
    expect(wordCount("")).toBe(0);
  });

  it("counts words across punctuation", () => {
    expect(wordCount("foo, bar! baz?")).toBe(3);
  });
});
