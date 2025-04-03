import { toKelvin } from "@/Math/toKelvin";

describe("toKelvin function", () => {
  it("should convert Celsius to Kelvin", () => {
    expect(toKelvin(0)).toBe(273.15);
    expect(toKelvin(26.85)).toBe(300);
    expect(toKelvin(100)).toBe(373.15);
  });

  it("should handle negative Celsius values", () => {
    expect(toKelvin(-40)).toBe(233.15);
    expect(toKelvin(-273.15)).toBe(0);
  });

  it("should handle extreme temperatures", () => {
    expect(toKelvin(1000)).toBe(1273.15);
    expect(toKelvin(-300)).toBe(-26.85); // Below absolute zero (physically impossible)
  });
});
