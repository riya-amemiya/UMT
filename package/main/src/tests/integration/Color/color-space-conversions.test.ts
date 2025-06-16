import {
  rgbaToHsla,
  hslaToRgba,
  rgbaToCmyk,
  cmykToRgba,
  rgbaToHexA,
  hexaToRgba,
} from "@/Color";

/**
 * Integration tests for Color conversion functions
 *
 * Tests the interaction between different color space conversions:
 * - Round-trip conversions (RGB → HSL → RGB)
 * - Multi-step color transformations
 * - Accuracy of conversions across color spaces
 */
describe("Integration test for color space conversions", () => {
  it("should perform round-trip conversion: RGBA → HSLA → RGBA", () => {
    const testColors = [
      { r: 255, g: 0, b: 0, a: 1 }, // Red
      { r: 0, g: 255, b: 0, a: 0.5 }, // Green with alpha
      { r: 0, g: 0, b: 255, a: 1 }, // Blue
      { r: 128, g: 128, b: 128, a: 1 }, // Gray
      { r: 255, g: 255, b: 255, a: 0.8 }, // White with alpha
    ];

    testColors.forEach((original) => {
      const hsla = rgbaToHsla(original);
      const converted = hslaToRgba(hsla.h, hsla.s, hsla.l, hsla.a);

      expect(converted.r).toBeCloseTo(original.r, 0);
      expect(converted.g).toBeCloseTo(original.g, 0);
      expect(converted.b).toBeCloseTo(original.b, 0);
      expect(converted.a).toBeCloseTo(original.a, 2);
    });
  });

  it("should perform round-trip conversion: RGBA → CMYK → RGBA", () => {
    const testColors = [
      { r: 255, g: 0, b: 0, a: 1 }, // Red
      { r: 0, g: 255, b: 0, a: 1 }, // Green
      { r: 0, g: 0, b: 255, a: 1 }, // Blue
      { r: 100, g: 150, b: 200, a: 0.7 }, // Custom color with alpha
    ];

    testColors.forEach((original) => {
      const cmyk = rgbaToCmyk(original);
      const converted = cmykToRgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, cmyk.a);

      expect(converted.r).toBeCloseTo(original.r, 0);
      expect(converted.g).toBeCloseTo(original.g, 0);
      expect(converted.b).toBeCloseTo(original.b, 0);
      expect(converted.a).toBeCloseTo(original.a, 2);
    });
  });

  it("should perform round-trip conversion: RGBA → HEX → RGBA", () => {
    const testColors = [
      { r: 255, g: 0, b: 0, a: 1 }, // Red
      { r: 0, g: 255, b: 0, a: 0.5 }, // Green with alpha
      { r: 128, g: 64, b: 192, a: 0.75 }, // Purple with alpha
    ];

    testColors.forEach((original) => {
      const hex = rgbaToHexA(original);
      const converted = hexaToRgba(hex);

      expect(converted.r).toBe(original.r);
      expect(converted.g).toBe(original.g);
      expect(converted.b).toBe(original.b);
      expect(converted.a).toBeCloseTo(original.a, 2);
    });
  });

  it("should handle multi-step color transformations", () => {
    const originalRgba = { r: 75, g: 150, b: 225, a: 0.9 };

    // RGBA → HSLA
    const hsla = rgbaToHsla(originalRgba);

    // HSLA → RGBA → CMYK
    const rgbaFromHsla = hslaToRgba(hsla.h, hsla.s, hsla.l, hsla.a);
    const cmyk = rgbaToCmyk(rgbaFromHsla);

    // CMYK → RGBA → HEX
    const rgbaFromCmyk = cmykToRgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, cmyk.a);
    const hex = rgbaToHexA(rgbaFromCmyk);

    // HEX → RGBA
    const finalRgba = hexaToRgba(hex);

    expect(finalRgba.r).toBeCloseTo(originalRgba.r, 0);
    expect(finalRgba.g).toBeCloseTo(originalRgba.g, 0);
    expect(finalRgba.b).toBeCloseTo(originalRgba.b, 0);
    expect(finalRgba.a).toBeCloseTo(originalRgba.a, 2);
  });

  it("should handle grayscale conversions across color spaces", () => {
    const grayLevels = [0, 64, 128, 192, 255];

    grayLevels.forEach((gray) => {
      const rgba = { r: gray, g: gray, b: gray, a: 1 };

      // Test HSLA conversion
      const hsla = rgbaToHsla(rgba);
      expect(hsla.s).toBe(0); // Saturation should be 0 for gray

      const rgbaFromHsla = hslaToRgba(hsla.h, hsla.s, hsla.l, hsla.a);
      expect(rgbaFromHsla.r).toBeCloseTo(gray, 0);
      expect(rgbaFromHsla.g).toBeCloseTo(gray, 0);
      expect(rgbaFromHsla.b).toBeCloseTo(gray, 0);

      // Test CMYK conversion
      const cmyk = rgbaToCmyk(rgba);
      const rgbaFromCmyk = cmykToRgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, cmyk.a);
      expect(rgbaFromCmyk.r).toBeCloseTo(gray, 0);
      expect(rgbaFromCmyk.g).toBeCloseTo(gray, 0);
      expect(rgbaFromCmyk.b).toBeCloseTo(gray, 0);
    });
  });

  it("should preserve alpha channel through conversions", () => {
    const alphaValues = [0, 0.25, 0.5, 0.75, 1];
    const baseColor = { r: 100, g: 150, b: 200 };

    alphaValues.forEach((alpha) => {
      const rgba = { ...baseColor, a: alpha };

      // Test through HSLA
      const hsla = rgbaToHsla(rgba);
      expect(hsla.a).toBeCloseTo(alpha, 2);

      // Test through CMYK
      const cmyk = rgbaToCmyk(rgba);
      expect(cmyk.a).toBeCloseTo(alpha, 2);

      // Test through HEX
      const hex = rgbaToHexA(rgba);
      const rgbaFromHex = hexaToRgba(hex);
      expect(rgbaFromHex.a).toBeCloseTo(alpha, 2);
    });
  });

  it("should handle edge cases in color conversions", () => {
    const edgeCases = [
      { r: 0, g: 0, b: 0, a: 1 }, // Black
      { r: 255, g: 255, b: 255, a: 1 }, // White
      { r: 255, g: 0, b: 255, a: 0 }, // Magenta with 0 alpha
    ];

    edgeCases.forEach((color) => {
      // Test all conversion paths
      const hsla = rgbaToHsla(color);
      const cmyk = rgbaToCmyk(color);
      const hex = rgbaToHexA(color);

      const fromHsla = hslaToRgba(hsla.h, hsla.s, hsla.l, hsla.a);
      const fromCmyk = cmykToRgba(cmyk.c, cmyk.m, cmyk.y, cmyk.k, cmyk.a);
      const fromHex = hexaToRgba(hex);

      // All conversions should yield similar results
      [fromHsla, fromCmyk, fromHex].forEach((converted) => {
        expect(converted.r).toBeCloseTo(color.r, 0);
        expect(converted.g).toBeCloseTo(color.g, 0);
        expect(converted.b).toBeCloseTo(color.b, 0);
        expect(converted.a).toBeCloseTo(color.a, 2);
      });
    });
  });

  it("should handle color mixing through conversions", () => {
    const color1 = { r: 255, g: 0, b: 0, a: 1 }; // Red
    const color2 = { r: 0, g: 0, b: 255, a: 1 }; // Blue

    // Convert to HSLA for mixing
    const hsla1 = rgbaToHsla(color1);
    const hsla2 = rgbaToHsla(color2);

    // Mix in HSLA space
    const mixedHsla = {
      h: (hsla1.h + hsla2.h) / 2,
      s: (hsla1.s + hsla2.s) / 2,
      l: (hsla1.l + hsla2.l) / 2,
      a: (hsla1.a + hsla2.a) / 2,
    };

    const mixedRgba = hslaToRgba(
      mixedHsla.h,
      mixedHsla.s,
      mixedHsla.l,
      mixedHsla.a,
    );
    const mixedHex = rgbaToHexA(mixedRgba);

    // Verify the mixed color has valid properties
    expect(mixedRgba.r).toBeGreaterThanOrEqual(0);
    expect(mixedRgba.b).toBeGreaterThanOrEqual(0);
    expect(mixedHex).toMatch(/^#[0-9a-f]{8}$/i);
  });
});
