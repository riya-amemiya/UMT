import { unitConverterInitialization } from "@/Unit/unitConverterInitialization";
const convertWaterUnit = unitConverterInitialization({
  L: 1,
  mL: 1000,
  cup: 4.227,
  m3: 0.001,
});
describe("convertWaterUnit", () => {
  it("should correctly convert between L and mL", () => {
    expect(convertWaterUnit(1, "L", "mL")).toBeCloseTo(1000);
    expect(convertWaterUnit(1000, "mL", "L")).toBeCloseTo(1);
  });

  it("should correctly convert between L and cup", () => {
    expect(convertWaterUnit(1, "L", "cup")).toBeCloseTo(4.227);
    expect(convertWaterUnit(1, "cup", "L")).toBeCloseTo(0.237);
  });

  it("should correctly convert between L and m3", () => {
    expect(convertWaterUnit(1, "m3", "L")).toBeCloseTo(1000);
    expect(convertWaterUnit(1000, "L", "m3")).toBeCloseTo(1);
  });

  it("should handle zero values correctly", () => {
    expect(convertWaterUnit(0, "L", "mL")).toBe(0);
    expect(convertWaterUnit(0, "cup", "m3")).toBe(0);
  });

  it("should return same value when converting to same unit", () => {
    expect(convertWaterUnit(5, "L", "L")).toBe(5);
    expect(convertWaterUnit(100, "mL", "mL")).toBe(100);
  });

  it("should maintain value in round-trip conversions", () => {
    const original = 5;
    const roundTrip = convertWaterUnit(
      convertWaterUnit(original, "L", "mL"),
      "mL",
      "L",
    );
    expect(roundTrip).toBeCloseTo(original);

    const cupRoundTrip = convertWaterUnit(
      convertWaterUnit(original, "cup", "m3"),
      "m3",
      "cup",
    );
    expect(cupRoundTrip).toBeCloseTo(original);
  });
});
