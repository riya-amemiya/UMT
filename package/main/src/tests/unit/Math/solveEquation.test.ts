import { solveEquation } from "@/Math/solveEquation";

describe("solveEquation function", () => {
  it("should solve a 2x2 system of linear equations (case 1)", () => {
    // System:
    // x + y = 4
    // x + 2y = 10
    expect(
      solveEquation(
        [
          [1, 1],
          [1, 2],
        ],
        [4, 10],
      ),
    ).toEqual([-2, 6]);
  });

  it("should solve a 2x2 system of linear equations (case 2)", () => {
    // System:
    // x + 6y = 33
    // x + y = 8
    expect(
      solveEquation(
        [
          [1, 6],
          [1, 1],
        ],
        [33, 8],
      ),
    ).toEqual([3, 5]);
  });

  it("should solve a 3x3 system of linear equations", () => {
    // System:
    // 5x - 4y + 6z = 8
    // 7x - 6y + 10z = 14
    // 4x + 9y + 7z = 74
    expect(
      solveEquation(
        [
          [5, -4, 6],
          [7, -6, 10],
          [4, 9, 7],
        ],
        [8, 14, 74],
      ),
    ).toEqual([2, 5, 3]);
  });
});
