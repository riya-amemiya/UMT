import { countOccurrences } from "@/String/countOccurrences";

describe("countOccurrences", () => {
  it("should count non-overlapping occurrences", () => {
    expect(countOccurrences("ababab", "ab")).toBe(3);
  });

  it("should not count overlapping occurrences", () => {
    expect(countOccurrences("aaaa", "aa")).toBe(2);
  });

  it("should count single-character occurrences", () => {
    expect(countOccurrences("banana", "a")).toBe(3);
  });

  it("should return 0 when the substring is absent", () => {
    expect(countOccurrences("hello", "z")).toBe(0);
  });

  it("should return 0 for an empty search string", () => {
    expect(countOccurrences("hello", "")).toBe(0);
  });

  it("should return 0 for an empty input string", () => {
    expect(countOccurrences("", "a")).toBe(0);
  });
});
