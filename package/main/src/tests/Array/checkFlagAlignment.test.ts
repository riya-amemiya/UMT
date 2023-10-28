import { checkFlagAlignment } from "@/Array/checkFlagAlignment";

describe("checkFlagAlignment", () => {
  it("横", () => {
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

  it("縦", () => {
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

  it("斜め(左上)", () => {
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

  it("斜め(右上)", () => {
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

  it("揃ってない時", () => {
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
  // 以下、大量データのテスト
  it("横(大量)", () => {
    const row = Array(1000).fill({ flag: true, value: 1 });
    const matrix = Array(1000).fill(row);
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("縦(大量)", () => {
    const row = Array(1000).fill({ flag: false, value: 1 });
    let matrix = Array(1000).fill(row);
    for (let i = 0; i < matrix.length; i++) {
      matrix[i][0].flag = true;
    }
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("斜め(大量)", () => {
    let matrix = Array.from({ length: 1000 }, () =>
      Array.from({ length: 1000 }, () => ({ flag: false, value: 1 })),
    );
    for (let i = 0; i < 1000; i++) {
      matrix[i][i].flag = true;
    }
    expect(checkFlagAlignment(matrix)).toBe(true);
  });

  it("揃ってない時(大量)", () => {
    const row = Array(1000).fill({ flag: false, value: 1 });
    const matrix = Array(1000).fill(row);
    expect(checkFlagAlignment(matrix)).toBe(false);
  });
});
