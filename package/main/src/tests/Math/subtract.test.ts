import { subtract } from "@/Math/subtract";
describe("subtract関数のテスト", () => {
  test.each([
    { a: [1, 1], expected: 0 },
    { a: [1.1, 1], expected: 0.1 },
    { a: [1, 1.1], expected: -0.1 },
    { a: [1, 1, 1], expected: -1 },
    { a: [5, 2, 1], expected: 2 },
    { a: [10, 1, 1, 1], expected: 7 },
  ])("$a の引き算の結果が $expected", ({ a, expected }) => {
    expect(subtract(...a)).toBe(expected);
  });
});
