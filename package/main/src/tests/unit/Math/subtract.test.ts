import { subtract } from "@/Math/subtract";

describe("subtract function", () => {
  test.each([
    { a: [1, 1], expected: 0 },
    { a: [1.1, 1], expected: 0.1 },
    { a: [1, 1.1], expected: -0.1 },
    { a: [1, 1, 1], expected: -1 },
    { a: [5, 2, 1], expected: 2 },
    { a: [10, 1, 1, 1], expected: 7 },
  ])("subtracting $a should result in $expected", ({ a, expected }) => {
    expect(subtract(...a)).toBe(expected);
  });

  describe("floating point precision", () => {
    it("should handle floating point precision correctly", () => {
      // These operations would normally have floating point errors
      expect(subtract(0.3, 0.1)).toBe(0.2);
      expect(subtract(0.7, 0.1)).toBe(0.6);
      expect(subtract(1.0, 0.9)).toBe(0.1);
    });

    it("should handle multiple decimal places", () => {
      expect(subtract(0.12345, 0.00001)).toBe(0.12344);
      expect(subtract(1.23456, 0.00001, 0.00002)).toBe(1.23453);
    });
  });
});
