import { addDuration } from "@/Date/addDuration";

describe("addDuration", () => {
  it("adds milliseconds", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 500, "ms");
    expect(result.getTime()).toBe(base.getTime() + 500);
  });

  it("adds seconds", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 30, "s");
    expect(result.getTime()).toBe(base.getTime() + 30_000);
  });

  it("adds minutes", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 5, "m");
    expect(result.getTime()).toBe(base.getTime() + 5 * 60_000);
  });

  it("adds hours", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 2, "h");
    expect(result.getTime()).toBe(base.getTime() + 2 * 3_600_000);
  });

  it("adds days", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 7, "d");
    expect(result.getTime()).toBe(base.getTime() + 7 * 86_400_000);
  });

  it("adds weeks", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = addDuration(base, 2, "w");
    expect(result.getTime()).toBe(base.getTime() + 14 * 86_400_000);
  });

  it("adds months and clamps to last day of target month", () => {
    const base = new Date(2025, 0, 31);
    const result = addDuration(base, 1, "M");
    expect(result.getFullYear()).toBe(2025);
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(28);
  });

  it("adds months across year boundary", () => {
    const base = new Date(2025, 11, 15);
    const result = addDuration(base, 2, "M");
    expect(result.getFullYear()).toBe(2026);
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(15);
  });

  it("subtracts via negative amount", () => {
    const base = new Date(2025, 2, 15);
    const result = addDuration(base, -1, "M");
    expect(result.getMonth()).toBe(1);
  });

  it("adds years preserving month and day", () => {
    const base = new Date(2024, 1, 29);
    const result = addDuration(base, 1, "y");
    expect(result.getFullYear()).toBe(2025);
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(28);
  });

  it("adds years across leap year boundary", () => {
    const base = new Date(2024, 1, 29);
    const result = addDuration(base, 4, "y");
    expect(result.getFullYear()).toBe(2028);
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(29);
  });

  it("does not mutate the input date", () => {
    const base = new Date(2025, 0, 1);
    const snapshot = base.getTime();
    addDuration(base, 5, "M");
    expect(base.getTime()).toBe(snapshot);
  });
});
