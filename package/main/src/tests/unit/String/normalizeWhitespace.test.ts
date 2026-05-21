import { normalizeWhitespace } from "@/String/normalizeWhitespace";

describe("normalizeWhitespace", () => {
  it("should collapse consecutive spaces into one", () => {
    expect(normalizeWhitespace("hello   world")).toBe("hello world");
  });

  it("should collapse mixed whitespace into single spaces", () => {
    expect(normalizeWhitespace("hello \t\n world")).toBe("hello world");
  });

  it("should trim leading and trailing whitespace", () => {
    expect(normalizeWhitespace("  hello world  ")).toBe("hello world");
  });

  it("should handle a string of only whitespace", () => {
    expect(normalizeWhitespace("  \t\n ")).toBe("");
  });

  it("should leave a single-spaced string unchanged", () => {
    expect(normalizeWhitespace("a b c")).toBe("a b c");
  });

  it("should handle empty string", () => {
    expect(normalizeWhitespace("")).toBe("");
  });
});
