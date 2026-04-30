import { words } from "@/String/words";

describe("words", () => {
  it("splits camelCase", () => {
    expect(words("helloWorld")).toEqual(["hello", "World"]);
  });

  it("splits acronym followed by word", () => {
    expect(words("XMLHttpRequest")).toEqual(["XML", "Http", "Request"]);
  });

  it("splits on dashes and underscores", () => {
    expect(words("foo-bar_baz")).toEqual(["foo", "bar", "baz"]);
  });

  it("splits on whitespace", () => {
    expect(words("hello world")).toEqual(["hello", "world"]);
  });

  it("returns empty for empty string", () => {
    expect(words("")).toEqual([]);
  });

  it("uses a custom pattern when provided", () => {
    expect(words("a1 b2 c3", /[a-z]\d/g)).toEqual(["a1", "b2", "c3"]);
  });

  it("returns empty array when custom pattern matches nothing", () => {
    expect(words("hello", /\d+/g)).toEqual([]);
  });
});
