import { radToDeg } from "@/Math/radToDeg";

describe("radToDeg", () => {
  describe("basic conversions", () => {
    it("should convert common angles correctly", () => {
      expect(radToDeg(0)).toBe(0);
      expect(radToDeg(Math.PI / 2)).toBe(90);
      expect(radToDeg(Math.PI)).toBe(180);
      expect(radToDeg((3 * Math.PI) / 2)).toBe(270);
      expect(radToDeg(2 * Math.PI)).toBe(360);
    });
  });

  describe("negative angles", () => {
    it("should handle negative angles correctly", () => {
      expect(radToDeg(-Math.PI / 2)).toBe(-90);
      expect(radToDeg(-Math.PI)).toBe(-180);
      expect(radToDeg((-3 * Math.PI) / 2)).toBe(-270);
      expect(radToDeg(-2 * Math.PI)).toBe(-360);
    });
  });

  describe("floating point angles", () => {
    it("should handle floating point angles correctly", () => {
      expect(radToDeg(Math.PI / 4)).toBeCloseTo(45);
      expect(radToDeg((Math.PI * 60.7) / 180)).toBeCloseTo(60.7);
      expect(radToDeg((Math.PI * 120.2) / 180)).toBeCloseTo(120.2);
    });
  });

  describe("angles greater than 2π", () => {
    it("should handle angles greater than 2π correctly", () => {
      expect(radToDeg((5 * Math.PI) / 2)).toBeCloseTo(450);
      expect(radToDeg(4 * Math.PI)).toBeCloseTo(720);
      expect(radToDeg(6 * Math.PI)).toBeCloseTo(1080);
    });
  });

  describe("special values", () => {
    it("should handle NaN and Infinity correctly", () => {
      expect(radToDeg(Number.NaN)).toBe(Number.NaN);
      expect(radToDeg(Number.POSITIVE_INFINITY)).toBe(Number.POSITIVE_INFINITY);
      expect(radToDeg(Number.NEGATIVE_INFINITY)).toBe(Number.NEGATIVE_INFINITY);
    });
  });

  describe("invalid inputs", () => {
    it("should handle invalid inputs appropriately", () => {
      expect(radToDeg("1.57" as any)).toBeCloseTo(89.95);
      expect(radToDeg([1.57] as any)).toBeCloseTo(89.95);
      expect(radToDeg({} as any)).toBe(Number.NaN);
    });
  });
});
