import { convertCurrency } from "@/Math/calculator/convertCurrency";

/**
 * Tests for the convertCurrency function
 */
describe("convertCurrency function", () => {
  // Case 1: No conversion rates provided
  test("should return original string when conversionRates is undefined", () => {
    expect(convertCurrency("¥1000")).toBe("¥1000");
  });

  // Case 2: Currency symbol doesn't match
  test("should return original string when currency symbol doesn't match", () => {
    expect(convertCurrency("¥1000", { $: 1.1 })).toBe("¥1000");
  });

  // Case 3: Currency symbol matches and conversion rate is a number
  test("should convert currency when symbol matches and rate is a number", () => {
    expect(convertCurrency("¥1000", { "¥": 1.1 })).toBe("1100");
  });

  // Case 4: Currency symbol matches but conversion rate is not a number
  test("should return original string when rate is not a number", () => {
    expect(convertCurrency("¥1000", { "¥": "invalid" })).toBe("¥1000");
  });

  // Case 5: Conversion result is NaN
  test("should return original string when conversion result is NaN", () => {
    expect(convertCurrency("¥NaN", { "¥": 1.1 })).toBe("¥NaN");
  });
});
