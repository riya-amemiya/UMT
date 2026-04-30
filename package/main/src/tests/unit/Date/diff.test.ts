import { diff } from "@/Date/diff";

describe("diff", () => {
  it("computes day difference", () => {
    expect(diff(new Date("2025-12-31"), new Date("2025-01-01"), "d")).toBe(364);
  });

  it("computes hour difference", () => {
    const left = new Date("2025-01-01T05:00:00Z");
    const right = new Date("2025-01-01T00:00:00Z");
    expect(diff(left, right, "h")).toBe(5);
  });

  it("computes minute difference", () => {
    const left = new Date("2025-01-01T00:30:00Z");
    const right = new Date("2025-01-01T00:00:00Z");
    expect(diff(left, right, "m")).toBe(30);
  });

  it("computes second difference", () => {
    const left = new Date("2025-01-01T00:00:30Z");
    const right = new Date("2025-01-01T00:00:00Z");
    expect(diff(left, right, "s")).toBe(30);
  });

  it("computes millisecond difference", () => {
    const left = new Date("2025-01-01T00:00:00.500Z");
    const right = new Date("2025-01-01T00:00:00Z");
    expect(diff(left, right, "ms")).toBe(500);
  });

  it("computes week difference", () => {
    const left = new Date("2025-01-15");
    const right = new Date("2025-01-01");
    expect(diff(left, right, "w")).toBe(2);
  });

  it("computes positive month difference adjusting for incomplete month", () => {
    expect(diff(new Date(2025, 2, 14), new Date(2025, 0, 15), "M")).toBe(1);
    expect(diff(new Date(2025, 2, 16), new Date(2025, 0, 15), "M")).toBe(2);
  });

  it("computes negative month difference adjusting for incomplete month", () => {
    expect(diff(new Date(2025, 0, 15), new Date(2025, 2, 14), "M")).toBe(-1);
    expect(diff(new Date(2025, 0, 15), new Date(2025, 2, 16), "M")).toBe(-2);
  });

  it("respects time-of-day for month boundary on positive diff", () => {
    expect(
      diff(
        new Date(2025, 1, 15, 10, 0, 0, 0),
        new Date(2025, 0, 15, 11, 0, 0, 0),
        "M",
      ),
    ).toBe(0);
  });

  it("respects time-of-day for month boundary on negative diff", () => {
    expect(
      diff(
        new Date(2025, 0, 15, 11, 0, 0, 0),
        new Date(2025, 1, 15, 10, 0, 0, 0),
        "M",
      ),
    ).toBe(0);
  });

  it("computes year difference", () => {
    expect(diff(new Date(2026, 0, 1), new Date(2025, 0, 1), "y")).toBe(1);
    expect(diff(new Date(2025, 0, 1), new Date(2026, 0, 1), "y")).toBe(-1);
  });

  it("returns zero for identical dates", () => {
    const date = new Date("2025-01-01");
    expect(diff(date, date, "d")).toBe(0);
    expect(diff(date, date, "M")).toBe(0);
    expect(diff(date, date, "y")).toBe(0);
  });

  it("truncates fractional fixed-unit differences toward zero", () => {
    const left = new Date(1500);
    const right = new Date(0);
    expect(diff(left, right, "s")).toBe(1);
    expect(diff(right, left, "s")).toBe(-1);
  });
});
