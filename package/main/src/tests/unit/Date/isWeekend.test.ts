import { isWeekend } from "@/Date/isWeekend";

describe("isWeekend", () => {
  it("returns true on Saturday", () => {
    expect(isWeekend(new Date(2025, 3, 19))).toBe(true);
  });

  it("returns true on Sunday", () => {
    expect(isWeekend(new Date(2025, 3, 20))).toBe(true);
  });

  it("returns false on Monday", () => {
    expect(isWeekend(new Date(2025, 3, 21))).toBe(false);
  });

  it("returns false on Friday", () => {
    expect(isWeekend(new Date(2025, 3, 18))).toBe(false);
  });
});
