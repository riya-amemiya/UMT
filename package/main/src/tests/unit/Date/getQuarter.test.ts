import { getQuarter } from "@/Date/getQuarter";

describe("getQuarter", () => {
  it("returns 1 for January through March", () => {
    expect(getQuarter(new Date(2025, 0, 1))).toBe(1);
    expect(getQuarter(new Date(2025, 2, 31))).toBe(1);
  });

  it("returns 2 for April through June", () => {
    expect(getQuarter(new Date(2025, 3, 1))).toBe(2);
    expect(getQuarter(new Date(2025, 5, 30))).toBe(2);
  });

  it("returns 3 for July through September", () => {
    expect(getQuarter(new Date(2025, 6, 1))).toBe(3);
    expect(getQuarter(new Date(2025, 8, 30))).toBe(3);
  });

  it("returns 4 for October through December", () => {
    expect(getQuarter(new Date(2025, 9, 1))).toBe(4);
    expect(getQuarter(new Date(2025, 11, 31))).toBe(4);
  });
});
