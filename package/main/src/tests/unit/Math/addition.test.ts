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
});
