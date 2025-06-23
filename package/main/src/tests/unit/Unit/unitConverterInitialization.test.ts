import { unitConverterInitialization } from "@/Unit/unitConverterInitialization";
// Example unit converter for length measurements
const convertLength = unitConverterInitialization({
  meters: 1, // base unit
  kilometers: 0.001,
  centimeters: 100,
  millimeters: 1000,
});

describe("Unit Converter", () => {
  it("converts between base unit and larger units", () => {
    expect(convertLength(1000, "meters", "kilometers")).toBeCloseTo(1);
    expect(convertLength(1, "kilometers", "meters")).toBeCloseTo(1000);
  });

  it("converts between base unit and smaller units", () => {
    expect(convertLength(1, "meters", "centimeters")).toBeCloseTo(100);
    expect(convertLength(100, "centimeters", "meters")).toBeCloseTo(1);
  });

  it("converts between different small units", () => {
    expect(convertLength(100, "centimeters", "millimeters")).toBeCloseTo(1000);
    expect(convertLength(1000, "millimeters", "centimeters")).toBeCloseTo(100);
  });

  it("handles zero values correctly", () => {
    expect(convertLength(0, "meters", "kilometers")).toBe(0);
    expect(convertLength(0, "millimeters", "centimeters")).toBe(0);
  });

  it("returns same value when converting to same unit", () => {
    expect(convertLength(5, "meters", "meters")).toBe(5);
    expect(convertLength(100, "centimeters", "centimeters")).toBe(100);
  });

  it("maintains precision in round-trip conversions", () => {
    const original = 5;
    const roundTrip = convertLength(
      convertLength(original, "meters", "kilometers"),
      "kilometers",
      "meters",
    );
    expect(roundTrip).toBeCloseTo(original);

    const smallUnitsRoundTrip = convertLength(
      convertLength(original, "centimeters", "millimeters"),
      "millimeters",
      "centimeters",
    );
    expect(smallUnitsRoundTrip).toBeCloseTo(original);
  });
});
