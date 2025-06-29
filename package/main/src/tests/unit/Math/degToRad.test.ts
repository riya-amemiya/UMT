import { degToRad } from "@/Math/degToRad";

describe("degToRad", () => {
  // Normal test cases
  test("degToRad: Normal Cases", () => {
    expect(degToRad(0)).toBeCloseTo(0);
    expect(degToRad(90)).toBeCloseTo(Math.PI / 2);
    expect(degToRad(180)).toBeCloseTo(Math.PI);
    expect(degToRad(270)).toBeCloseTo((3 * Math.PI) / 2);
    expect(degToRad(360)).toBeCloseTo(2 * Math.PI);
  });

  // Negative angles
  test("degToRad: Negative Angles", () => {
    expect(degToRad(-90)).toBeCloseTo(-Math.PI / 2);
    expect(degToRad(-180)).toBeCloseTo(-Math.PI);
    expect(degToRad(-270)).toBeCloseTo((-3 * Math.PI) / 2);
    expect(degToRad(-360)).toBeCloseTo(-2 * Math.PI);
  });

  // Floating point angles
  test("degToRad: Floating Point Angles", () => {
    expect(degToRad(45.5)).toBeCloseTo(Math.PI / 4 + Math.PI / 360);
    expect(degToRad(60.7)).toBeCloseTo((Math.PI * 60.7) / 180);
    expect(degToRad(120.2)).toBeCloseTo((Math.PI * 120.2) / 180);
  });

  // Angles greater than 360 and 1000 degrees
  test("degToRad: Angles Greater Than 360 and 1000 Degrees", () => {
    expect(degToRad(450)).toBeCloseTo((5 * Math.PI) / 2);
    expect(degToRad(720)).toBeCloseTo(4 * Math.PI);
    expect(degToRad(1080)).toBeCloseTo(6 * Math.PI);
    expect(degToRad(1440)).toBeCloseTo(8 * Math.PI);
  });

  // NaN and Infinity cases
  test("degToRad: NaN and Infinity", () => {
    expect(degToRad(Number.NaN)).toBe(Number.NaN);
    expect(degToRad(Number.POSITIVE_INFINITY)).toBe(Number.POSITIVE_INFINITY);
    expect(degToRad(Number.NEGATIVE_INFINITY)).toBe(Number.NEGATIVE_INFINITY);
  });

  // Invalid inputs
  test("degToRad: Invalid Inputs", () => {
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(degToRad("90" as any)).toBeCloseTo(Math.PI / 2);
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(degToRad([90] as any)).toBeCloseTo(Math.PI / 2);
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(degToRad({} as any)).toBe(Number.NaN);
  });
});
