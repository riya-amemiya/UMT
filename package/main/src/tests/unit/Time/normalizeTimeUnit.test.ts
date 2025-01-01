import { normalizeTimeUnit } from "@/Time/normalizeTimeUnit";

describe("normalizeTimeUnit", () => {
  test("converts long format to long format", () => {
    expect(normalizeTimeUnit("milliseconds", "long")).toBe("milliseconds");
    expect(normalizeTimeUnit("seconds", "long")).toBe("seconds");
    expect(normalizeTimeUnit("minutes", "long")).toBe("minutes");
    expect(normalizeTimeUnit("hours", "long")).toBe("hours");
  });

  test("converts long format to short format", () => {
    expect(normalizeTimeUnit("milliseconds", "short")).toBe("ms");
    expect(normalizeTimeUnit("seconds", "short")).toBe("s");
    expect(normalizeTimeUnit("minutes", "short")).toBe("m");
    expect(normalizeTimeUnit("hours", "short")).toBe("h");
  });

  test("converts short format to long format", () => {
    expect(normalizeTimeUnit("ms", "long")).toBe("milliseconds");
    expect(normalizeTimeUnit("s", "long")).toBe("seconds");
    expect(normalizeTimeUnit("m", "long")).toBe("minutes");
    expect(normalizeTimeUnit("h", "long")).toBe("hours");
  });

  test("converts short format to short format", () => {
    expect(normalizeTimeUnit("ms", "short")).toBe("ms");
    expect(normalizeTimeUnit("s", "short")).toBe("s");
    expect(normalizeTimeUnit("m", "short")).toBe("m");
    expect(normalizeTimeUnit("h", "short")).toBe("h");
  });
});
