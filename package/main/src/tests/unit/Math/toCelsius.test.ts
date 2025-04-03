import { toCelsius } from "@/Math/toCelsius";

describe("toCelsius function", () => {
  it("should convert Kelvin to Celsius", () => {
    expect(toCelsius(273.15)).toBe(0);
    expect(toCelsius(300)).toBe(26.85);
    expect(toCelsius(373.15)).toBe(100);
    expect(toCelsius(0)).toBe(-273.15);
  });

  it("should handle non-standard temperatures", () => {
    expect(toCelsius(32)).toBe(-241.15);
    expect(toCelsius(1000)).toBe(726.85);
  });

  it("should handle negative Kelvin values (though physically impossible)", () => {
    expect(toCelsius(-100)).toBe(-373.15);
  });
});
