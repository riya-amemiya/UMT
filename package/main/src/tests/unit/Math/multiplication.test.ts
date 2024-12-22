import { multiplication } from "@/Math/multiplication";
describe("multiplication", () => {
  test.each([
    [[1, 1], 1],
    [[1, 2], 2],
    [[1.1, 2.2], 2.42],
    [[1, 2, 3], 6],
    [[2, 2, 2], 8],
    [[0.5, 2, 2], 2],
  ])("multiplication(%s) equals %s", (args, expected) => {
    expect(multiplication(...args)).toBeCloseTo(expected);
  });
});
