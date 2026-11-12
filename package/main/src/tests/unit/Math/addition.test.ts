import { addition } from "@/Math/addition";

describe("addition", () => {
  const testCases = [
    { args: [2, 3], expected: 5 },
    { args: [-2, -3], expected: -5 },
    { args: [2, -3], expected: -1 },
    { args: [0.1, 0.2], expected: 0.3, closeTo: true },
    { args: [-0.1, -0.2], expected: -0.3, closeTo: true },
    { args: [0.1, -0.2], expected: -0.1, closeTo: true },
    { args: [2, 0.3], expected: 2.3, closeTo: true },
    { args: [-2, -0.3], expected: -2.3, closeTo: true },
    { args: [2, -0.3], expected: 1.7, closeTo: true },
    { args: [-2, 0.3], expected: -1.7, closeTo: true },
    { args: [1, 2, 3], expected: 6 },
    { args: [-1, -2, -3], expected: -6 },
    { args: [0.1, 0.2, 0.3], expected: 0.6, closeTo: true },
    { args: [2, -3, 1], expected: 0 },
    { args: [-2, 0.5, 1.5], expected: 0, closeTo: true },
    { args: [], expected: 0 },
    { args: [7], expected: 7 },
    { args: [1.0, 2.0, 3.0], expected: 6 },
    { args: [0, 0, 0], expected: 0 },
    { args: [1_000_000_000, 1_000_000_000], expected: 2_000_000_000 },
  ];

  for (const { args, expected, closeTo } of testCases) {
    it(`should add ${args.length} numbers: ${args.join(", ")}`, () => {
      const result = addition(...args);
      if (closeTo) {
        expect(result).toBeCloseTo(expected);
      } else {
        expect(result).toBe(expected);
      }
    });
  }

  it("should match native + for integers", () => {
    expect(addition(2, 3)).toBe(2 + 3);
    expect(addition(1, 2, 3, 4, 5)).toBe(1 + 2 + 3 + 4 + 5);
    expect(addition(-10, 4, 6)).toBe(-10 + 4 + 6);
  });

  it("should keep 0.1 + 0.2 exact", () => {
    expect(addition(0.1, 0.2)).toBe(0.3);
  });

  it("should not round away Number.MIN_VALUE", () => {
    expect(addition(Number.MIN_VALUE, Number.MIN_VALUE)).toBe(
      2 * Number.MIN_VALUE,
    );
    expect(addition(Number.MIN_VALUE, -Number.MIN_VALUE)).toBe(0);
  });
});
