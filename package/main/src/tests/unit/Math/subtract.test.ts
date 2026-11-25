import { subtract } from "@/Math/subtract";

describe("subtract function", () => {
  test.each([
    { a: [1, 1], expected: 0 },
    { a: [1.1, 1], expected: 0.1 },
    { a: [1, 1.1], expected: -0.1 },
    { a: [1, 1, 1], expected: -1 },
    { a: [5, 2, 1], expected: 2 },
    { a: [10, 1, 1, 1], expected: 7 },
    { a: [7], expected: 7 },
    { a: [0, 0, 0], expected: 0 },
    { a: [1.0, 2.0, 3.0], expected: -4 },
    { a: [1_000_000_000, 1_000_000_000], expected: 0 },
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
      expect(subtract(0.123_45, 0.000_01)).toBe(0.123_44);
      expect(subtract(1.234_56, 0.000_01, 0.000_02)).toBe(1.234_53);
    });

    it("should keep mixed integer and decimal exact", () => {
      expect(subtract(2, 0.3)).toBe(1.7);
    });
  });

  it("should match native - for integers", () => {
    expect(subtract(2, 3)).toBe(2 - 3);
    expect(subtract(5, 2, 1)).toBe(5 - 2 - 1);
    expect(subtract(10, 1, 1, 1)).toBe(10 - 1 - 1 - 1);
    expect(subtract(-10, 4, 6)).toBe(-10 - 4 - 6);
  });

  it("should keep 0.3 - 0.1 exact", () => {
    expect(subtract(0.3, 0.1)).toBe(0.2);
  });
});
