import { formatDuration } from "@/Time/formatDuration";

describe("formatDuration", () => {
  it("returns 0s for zero", () => {
    expect(formatDuration(0)).toBe("0s");
    expect(formatDuration(-0)).toBe("0s");
  });

  it("formats single units", () => {
    expect(formatDuration(45_000)).toBe("45s");
    expect(formatDuration(60_000)).toBe("1m");
    expect(formatDuration(3_600_000)).toBe("1h");
    expect(formatDuration(86_400_000)).toBe("1d");
    expect(formatDuration(250)).toBe("250ms");
  });

  it("omits zero units", () => {
    expect(formatDuration(5_400_000)).toBe("1h 30m");
    expect(formatDuration(183_600_000)).toBe("2d 3h");
    expect(formatDuration(86_400_005)).toBe("1d 5ms");
  });

  it("formats every unit", () => {
    expect(formatDuration(93_784_005)).toBe("1d 2h 3m 4s 5ms");
  });

  it("does not cap days", () => {
    expect(formatDuration(400 * 86_400_000)).toBe("400d");
  });

  it("truncates fractional milliseconds", () => {
    expect(formatDuration(1500.9)).toBe("1s 500ms");
    expect(formatDuration(0.5)).toBe("0s");
  });

  it("keeps a leading minus for negative durations", () => {
    expect(formatDuration(-90_000)).toBe("-1m 30s");
    expect(formatDuration(-5_400_000)).toBe("-1h 30m");
  });

  it("returns 0s without a sign when the negative value truncates to zero", () => {
    expect(formatDuration(-0.5)).toBe("0s");
  });
});
