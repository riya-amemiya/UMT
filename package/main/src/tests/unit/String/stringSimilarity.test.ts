import { stringSimilarity } from "@/String/stringSimilarity";

describe("stringSimilarity", () => {
  it("should return 1 for identical strings", () => {
    expect(stringSimilarity("hello", "hello")).toBe(1);
    expect(stringSimilarity("world", "world")).toBe(1);
  });

  it("should return 0 for empty strings", () => {
    expect(stringSimilarity("", "hello")).toBe(0);
    expect(stringSimilarity("hello", "")).toBe(0);
    expect(stringSimilarity("", "")).toBe(1); // both empty are identical
  });

  it("should calculate similarity for similar strings", () => {
    const similarity1 = stringSimilarity("cat", "bat");
    expect(similarity1).toBeCloseTo(0.667, 3); // 1 - (1/3)

    const similarity2 = stringSimilarity("kitten", "sitting");
    expect(similarity2).toBeCloseTo(0.571, 3); // 1 - (3/7)
  });

  it("should return 0 for completely different strings", () => {
    const similarity = stringSimilarity("abc", "xyz");
    expect(similarity).toBe(0); // 1 - (3/3)
  });

  it("should handle strings of different lengths", () => {
    const similarity1 = stringSimilarity("cat", "cats");
    expect(similarity1).toBe(0.75); // 1 - (1/4)

    const similarity2 = stringSimilarity("hello", "helo");
    expect(similarity2).toBe(0.8); // 1 - (1/5)
  });

  it("should be case sensitive", () => {
    const similarity = stringSimilarity("Hello", "hello");
    expect(similarity).toBe(0.8); // 1 - (1/5)
  });

  it("should handle unicode characters", () => {
    const similarity1 = stringSimilarity("café", "cafe");
    expect(similarity1).toBe(0.75); // 1 - (1/4)

    const similarity2 = stringSimilarity("😀😁", "😀😂");
    expect(similarity2).toBe(0.75); // 1 - (1/4) - emoji counts as 2 characters each
  });

  it("should return values between 0 and 1", () => {
    const testCases = [
      ["hello", "world"],
      ["test", "testing"],
      ["a", "b"],
      ["similar", "similarity"],
    ];

    for (const [str1, str2] of testCases) {
      const similarity = stringSimilarity(str1, str2);
      expect(similarity).toBeGreaterThanOrEqual(0);
      expect(similarity).toBeLessThanOrEqual(1);
    }
  });
});
