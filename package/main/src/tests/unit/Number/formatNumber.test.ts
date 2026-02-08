import { formatNumber } from "@/Number/formatNumber";

describe("formatNumber", () => {
  it("should format a number with default options", () => {
    const result = formatNumber(1_234_567.89, { locale: "en-US" });
    expect(result).toBe("1,234,567.89");
  });

  it("should format with specified locale", () => {
    const result = formatNumber(1_234_567.89, { locale: "de-DE" });
    expect(result).toBe("1.234.567,89");
  });

  it("should format as percentage", () => {
    const result = formatNumber(0.75, {
      style: "percent",
      locale: "en-US",
    });
    expect(result).toBe("75%");
  });

  it("should format as currency", () => {
    const result = formatNumber(1234.5, {
      style: "currency",
      currency: "USD",
      locale: "en-US",
    });
    expect(result).toBe("$1,234.50");
  });

  it("should respect minimumFractionDigits", () => {
    const result = formatNumber(42, {
      minimumFractionDigits: 2,
      locale: "en-US",
    });
    expect(result).toBe("42.00");
  });

  it("should respect maximumFractionDigits", () => {
    const result = formatNumber(3.141_58, {
      maximumFractionDigits: 2,
      locale: "en-US",
    });
    expect(result).toBe("3.14");
  });

  it("should handle zero", () => {
    const result = formatNumber(0, { locale: "en-US" });
    expect(result).toBe("0");
  });

  it("should handle negative numbers", () => {
    const result = formatNumber(-1234.56, { locale: "en-US" });
    expect(result).toBe("-1,234.56");
  });

  it("should format with Japanese yen currency", () => {
    const result = formatNumber(1234, {
      style: "currency",
      currency: "JPY",
      locale: "ja-JP",
    });
    expect(result).toBe("￥1,234");
  });

  it("should format with no options", () => {
    const result = formatNumber(1234.56);
    expect(typeof result).toBe("string");
    expect(result).toContain("1");
  });
});
