import { fuzzySearch } from "@/String/fuzzySearch";

describe("fuzzySearch", () => {
  it("should perform fuzzy string matching according to JSDoc example", () => {
    const result = fuzzySearch("hello", ["hello", "world", "helo", "help"]);
    expect(result).toContainEqual({ item: "hello", score: 1 });
    expect(result).toContainEqual({ item: "helo", score: 0.8 });
    expect(result).toContainEqual({ item: "help", score: 0.6 });
    expect(result).toHaveLength(3);
  });

  it("should find exact matches", () => {
    const result = fuzzySearch("test", ["test", "best", "rest"]);
    expect(result[0]).toEqual({ item: "test", score: 1 });
  });

  it("should sort results by score descending", () => {
    const result = fuzzySearch("test", ["test", "tests", "testing"]);
    for (let i = 0; i < result.length - 1; i++) {
      expect(result[i].score).toBeGreaterThanOrEqual(result[i + 1].score);
    }
  });

  it("should use custom threshold", () => {
    const highThreshold = fuzzySearch("hello", ["hello", "helo", "help"], 0.9);
    const lowThreshold = fuzzySearch("hello", ["hello", "helo", "help"], 0.3);
    expect(highThreshold.length).toBeLessThanOrEqual(lowThreshold.length);
  });

  it("should be case insensitive", () => {
    const result = fuzzySearch("Hello", ["HELLO", "hello", "Hello"]);
    expect(result).toHaveLength(3);
    expect(result.every((r) => r.score === 1)).toBe(true);
  });

  it("should handle empty query", () => {
    const result = fuzzySearch("", ["hello", "world"]);
    expect(result).toEqual([]);
  });

  it("should handle empty items array", () => {
    const result = fuzzySearch("hello", []);
    expect(result).toEqual([]);
  });

  it("should filter out items below threshold", () => {
    const result = fuzzySearch("hello", ["world", "xyz"], 0.8);
    expect(result).toEqual([]);
  });

  it("should handle single character matches", () => {
    const result = fuzzySearch("a", ["a", "b", "ab"]);
    expect(result).toContainEqual({ item: "a", score: 1 });
  });

  it("should handle special characters", () => {
    const result = fuzzySearch("hello!", ["hello!", "hello"]);
    expect(result[0]).toEqual({ item: "hello!", score: 1 });
  });
});
