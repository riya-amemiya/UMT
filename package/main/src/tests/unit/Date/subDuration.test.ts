import { subDuration } from "@/Date/subDuration";

describe("subDuration", () => {
  it("subtracts milliseconds", () => {
    const base = new Date("2025-01-01T00:00:00.000Z");
    const result = subDuration(base, 1000, "ms");
    expect(result.getTime()).toBe(base.getTime() - 1000);
  });

  it("subtracts months and clamps to last day", () => {
    const base = new Date(2025, 2, 31);
    const result = subDuration(base, 1, "M");
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(28);
  });

  it("subtracts years across leap year", () => {
    const base = new Date(2025, 1, 28);
    const result = subDuration(base, 1, "y");
    expect(result.getFullYear()).toBe(2024);
    expect(result.getMonth()).toBe(1);
    expect(result.getDate()).toBe(28);
  });

  it("subtracts seconds", () => {
    const base = new Date("2025-06-01T12:00:00.000Z");
    const result = subDuration(base, 30, "s");
    expect(result.getTime()).toBe(base.getTime() - 30_000);
  });

  it("subtracts minutes", () => {
    const base = new Date("2025-06-01T12:00:00.000Z");
    const result = subDuration(base, 5, "m");
    expect(result.getTime()).toBe(base.getTime() - 300_000);
  });

  it("subtracts hours", () => {
    const base = new Date("2025-06-01T12:00:00.000Z");
    const result = subDuration(base, 1, "h");
    expect(result.getTime()).toBe(base.getTime() - 3_600_000);
  });

  it("subtracts days", () => {
    const base = new Date("2025-06-01T12:00:00.000Z");
    const result = subDuration(base, 1, "d");
    expect(result.getTime()).toBe(base.getTime() - 86_400_000);
  });

  it("subtracts weeks", () => {
    const base = new Date("2025-06-15T12:00:00.000Z");
    const result = subDuration(base, 1, "w");
    expect(result.getTime()).toBe(base.getTime() - 7 * 86_400_000);
  });
});
