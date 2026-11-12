import { damerauLevenshteinDistance } from "@/String/damerauLevenshteinDistance";
import { levenshteinDistance } from "@/String/levenshteinDistance";

describe("damerauLevenshteinDistance", () => {
  it("should compute distances from JSDoc examples", () => {
    expect(damerauLevenshteinDistance("ca", "ac")).toBe(1);
    expect(damerauLevenshteinDistance("kitten", "sitting")).toBe(3);
    expect(damerauLevenshteinDistance("ca", "abc")).toBe(3);
  });

  it("should return 0 for identical strings", () => {
    expect(damerauLevenshteinDistance("hello", "hello")).toBe(0);
    expect(damerauLevenshteinDistance("", "")).toBe(0);
  });

  it("should handle empty strings", () => {
    expect(damerauLevenshteinDistance("", "abc")).toBe(3);
    expect(damerauLevenshteinDistance("abc", "")).toBe(3);
  });

  it("should count an adjacent transposition as one edit", () => {
    expect(damerauLevenshteinDistance("ab", "ba")).toBe(1);
    expect(levenshteinDistance("ab", "ba")).toBe(2);
    expect(damerauLevenshteinDistance("abcd", "acbd")).toBe(1);
    expect(damerauLevenshteinDistance("abcdef", "badcfe")).toBe(3);
  });

  it("should count single edits", () => {
    expect(damerauLevenshteinDistance("a", "b")).toBe(1);
    expect(damerauLevenshteinDistance("cat", "cats")).toBe(1);
    expect(damerauLevenshteinDistance("cats", "cat")).toBe(1);
  });

  it("should keep the cheaper path when a transposition does not help", () => {
    expect(damerauLevenshteinDistance("aax", "aay")).toBe(1);
  });

  it("should be symmetric", () => {
    expect(damerauLevenshteinDistance("abc", "ca")).toBe(3);
    expect(damerauLevenshteinDistance("sitting", "kitten")).toBe(3);
  });

  it("should compare UTF-16 code units", () => {
    expect(damerauLevenshteinDistance("\u{1f600}", "\uDE00\uD83D")).toBe(1);
  });
});
