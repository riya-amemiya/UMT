import { formatRelative } from "@/Date/formatRelative";

describe("formatRelative", () => {
  it("returns 'now' for delta below one second", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() + 500);
    expect(formatRelative(target, base, "en")).toBe("now");
  });

  it("formats seconds in the past", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() - 5000);
    expect(formatRelative(target, base, "en")).toBe("5 seconds ago");
  });

  it("formats minutes in the future", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() + 5 * 60_000);
    expect(formatRelative(target, base, "en")).toBe("in 5 minutes");
  });

  it("formats hours", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() - 3 * 3_600_000);
    expect(formatRelative(target, base, "en")).toBe("3 hours ago");
  });

  it("formats days", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() + 86_400_000);
    expect(formatRelative(target, base, "en")).toBe("tomorrow");
  });

  it("formats weeks", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() - 14 * 86_400_000);
    expect(formatRelative(target, base, "en")).toBe("2 weeks ago");
  });

  it("formats months", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() + 90 * 86_400_000);
    expect(formatRelative(target, base, "en")).toBe("in 3 months");
  });

  it("formats years", () => {
    const base = new Date("2025-04-15T12:00:00.000Z");
    const target = new Date(base.getTime() - 2 * 365 * 86_400_000);
    expect(formatRelative(target, base, "en")).toBe("2 years ago");
  });

  it("uses default base date when omitted", () => {
    const target = new Date(Date.now() + 5000);
    const result = formatRelative(target, undefined, "en");
    expect(typeof result).toBe("string");
    expect(result.length).toBeGreaterThan(0);
  });
});
