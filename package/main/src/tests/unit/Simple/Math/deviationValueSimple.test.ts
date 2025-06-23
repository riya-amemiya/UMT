import { deviationValueSimple } from "@/Simple/Math/deviationValueSimple";

describe("deviationValueSimple", () => {
  describe("with explicit values", () => {
    it("calculates deviation value using given average and standard deviation", () => {
      // When value equals average (no deviation)
      expect(deviationValueSimple(50, 50, 10)).toBe(50);

      // One standard deviation above average
      expect(deviationValueSimple(60, 50, 10)).toBe(60);

      // One standard deviation below average
      expect(deviationValueSimple(40, 50, 10)).toBe(40);

      // When standard deviation is 0
      expect(deviationValueSimple(100, 50, 0)).toBe(50);
    });

    it("throws TypeError when standard deviation is missing", () => {
      // @ts-expect-error Testing runtime error
      expect(() => deviationValueSimple(50, 50)).toThrow(TypeError);
    });
  });

  describe("with array input", () => {
    it("calculates deviation value using array of reference scores", () => {
      // Using simple array [40, 50, 60]
      // mean = 50
      // population standard deviation ≈ 8.165
      const scores = [40, 50, 60];
      expect(deviationValueSimple(60, scores)).toBeCloseTo(62.25, 2); // +1.225 SD
      expect(deviationValueSimple(50, scores)).toBe(50); // mean
      expect(deviationValueSimple(40, scores)).toBeCloseTo(37.75, 2); // -1.225 SD
    });

    it("returns 50 for any value when all reference values are the same", () => {
      const sameScores = [50, 50, 50];
      expect(deviationValueSimple(50, sameScores)).toBe(50);
      expect(deviationValueSimple(0, sameScores)).toBe(50);
      expect(deviationValueSimple(100, sameScores)).toBe(50);
    });
  });
});
