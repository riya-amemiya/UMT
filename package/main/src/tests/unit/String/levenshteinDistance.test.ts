import { levenshteinDistance } from "@/String/levenshteinDistance";

describe("levenshteinDistance", () => {
  it("should return 0 for identical strings", () => {
    expect(levenshteinDistance("hello", "hello")).toBe(0);
    expect(levenshteinDistance("", "")).toBe(0);
  });

  it("should handle empty strings", () => {
    expect(levenshteinDistance("", "hello")).toBe(5);
    expect(levenshteinDistance("hello", "")).toBe(5);
  });

  it("should calculate distance for single character differences", () => {
    expect(levenshteinDistance("cat", "bat")).toBe(1); // substitution
    expect(levenshteinDistance("cat", "cats")).toBe(1); // insertion
    expect(levenshteinDistance("cats", "cat")).toBe(1); // deletion
  });

  it("should calculate distance for multiple differences", () => {
    expect(levenshteinDistance("kitten", "sitting")).toBe(3);
    expect(levenshteinDistance("saturday", "sunday")).toBe(3);
  });

  it("should handle completely different strings", () => {
    expect(levenshteinDistance("abc", "xyz")).toBe(3);
    expect(levenshteinDistance("hello", "world")).toBe(4);
  });

  it("should be case sensitive", () => {
    expect(levenshteinDistance("Hello", "hello")).toBe(1);
    expect(levenshteinDistance("ABC", "abc")).toBe(3);
  });

  it("should handle unicode characters", () => {
    expect(levenshteinDistance("café", "cafe")).toBe(1);
    expect(levenshteinDistance("😀", "😁")).toBe(1);
    expect(levenshteinDistance("こんにちは", "こんばんは")).toBe(2);
  });
});
