import { checkFlagAlignment } from "@/Array/checkFlagAlignment";

describe("checkFlagAlignment", () => {
  it("should detect horizontal alignment", () => {
    const matrix = [
      [
        { value: 10, flag: true },
        { value: 20, flag: true },
        { value: 30, flag: true },
      ],
      [
        { value: 40, flag: false },
        { value: 50, flag: false },
        { value: 60, flag: false },
      ],
      [
        { value: 70, flag: false },
        { value: 80, flag: false },
        { value: 90, flag: false },
      ],
    ];
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should detect vertical alignment", () => {
    const matrix = [
      [
        { value: 10, flag: false },
        { value: 20, flag: true },
        { value: 30, flag: false },
      ],
      [
        { value: 40, flag: false },
        { value: 50, flag: true },
        { value: 60, flag: false },
      ],
      [
        { value: 70, flag: false },
        { value: 80, flag: true },
        { value: 90, flag: false },
      ],
    ];
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should detect diagonal alignment (top-left to bottom-right)", () => {
    const matrix = [
      [
        { value: 10, flag: true },
        { value: 20, flag: false },
        { value: 30, flag: false },
      ],
      [
        { value: 40, flag: false },
        { value: 50, flag: true },
        { value: 60, flag: false },
      ],
      [
        { value: 70, flag: false },
        { value: 80, flag: false },
        { value: 90, flag: true },
      ],
    ];
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should detect diagonal alignment (top-right to bottom-left)", () => {
    const matrix = [
      [
        { value: 10, flag: false },
        { value: 20, flag: false },
        { value: 30, flag: true },
      ],
      [
        { value: 40, flag: false },
        { value: 50, flag: true },
        { value: 60, flag: false },
      ],
      [
        { value: 70, flag: true },
        { value: 80, flag: false },
        { value: 90, flag: false },
      ],
    ];
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should return false when no alignment is found", () => {
    const matrix = [
      [
        { value: 10, flag: false },
        { value: 20, flag: true },
        { value: 30, flag: false },
      ],
      [
        { value: 40, flag: false },
        { value: 50, flag: true },
        { value: 60, flag: false },
      ],
      [
        { value: 70, flag: false },
        { value: 80, flag: false },
        { value: 90, flag: false },
      ],
    ];
    expect(checkFlagAlignment(matrix)).toBe(false);
  });
  // Tests with large datasets
  it("should detect horizontal alignment with large dataset", () => {
    const row = Array(1000).fill({ flag: true, value: 1 });
    const matrix = Array(1000).fill(row);
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should detect vertical alignment with large dataset", () => {
    const row = Array(1000).fill({ flag: false, value: 1 });
    const matrix = Array(1000).fill(row);
    for (let i = 0; i < matrix.length; i++) {
      matrix[i][0].flag = true;
    }
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should detect diagonal alignment with large dataset", () => {
    const matrix = Array.from({ length: 1000 }, () =>
      Array.from({ length: 1000 }, () => ({ flag: false, value: 1 })),
    );
    for (let i = 0; i < 1000; i++) {
      matrix[i][i].flag = true;
    }
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("should return false when no alignment is found in large dataset", () => {
    const row = Array(1000).fill({ flag: false, value: 1 });
    const matrix = Array(1000).fill(row);
    expect(checkFlagAlignment(matrix)).toBe(false);
  });
});
