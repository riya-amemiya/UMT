import { formatBytes } from "@/Number/formatBytes";

describe("formatBytes", () => {
  it("formats zero as bytes", () => {
    expect(formatBytes(0)).toBe("0 B");
  });

  it("keeps values below 1024 in bytes", () => {
    expect(formatBytes(1)).toBe("1 B");
    expect(formatBytes(512)).toBe("512 B");
    expect(formatBytes(1023)).toBe("1023 B");
  });

  it("keeps fractional bytes below 1 in bytes", () => {
    expect(formatBytes(0.5)).toBe("0.5 B");
  });

  it("uses each unit at its 1024 boundary", () => {
    expect(formatBytes(1024)).toBe("1 KB");
    expect(formatBytes(1024 ** 2)).toBe("1 MB");
    expect(formatBytes(1024 ** 3)).toBe("1 GB");
    expect(formatBytes(1024 ** 4)).toBe("1 TB");
    expect(formatBytes(1024 ** 5)).toBe("1 PB");
  });

  it("stays in PB for values of 1024 PB or more", () => {
    expect(formatBytes(1024 ** 6)).toBe("1024 PB");
  });

  it("rounds to two decimals by default", () => {
    expect(formatBytes(1536)).toBe("1.5 KB");
    expect(formatBytes(1_234_567)).toBe("1.18 MB");
    expect(formatBytes(1_048_575)).toBe("1024 KB");
  });

  it("respects custom decimals", () => {
    expect(formatBytes(1_234_567, 0)).toBe("1 MB");
    expect(formatBytes(1_234_567, 1)).toBe("1.2 MB");
    expect(formatBytes(1_234_567, 4)).toBe("1.1774 MB");
  });

  it("formats negative values with the unit of their absolute value", () => {
    expect(formatBytes(-2048)).toBe("-2 KB");
    expect(formatBytes(-1536)).toBe("-1.5 KB");
  });

  it("does not print a negative zero", () => {
    expect(formatBytes(-0)).toBe("0 B");
    expect(formatBytes(-0.001)).toBe("0 B");
  });
});
