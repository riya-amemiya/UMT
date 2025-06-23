import { factorize } from "@/Math/factorize";
describe("factorize", () => {
  describe("basic factorization", () => {
    test("should correctly factorize composite numbers", () => {
      expect(factorize(12)).toEqual([2, 2, 3]);
      expect(factorize(14)).toEqual([2, 7]);
      expect(factorize(15)).toEqual([3, 5]);
      expect(factorize(18)).toEqual([2, 3, 3]);
      expect(factorize(20)).toEqual([2, 2, 5]);
    });

    test("should handle powers of the same prime", () => {
      expect(factorize(16)).toEqual([2, 2, 2, 2]);
      expect(factorize(27)).toEqual([3, 3, 3]);
      expect(factorize(32)).toEqual([2, 2, 2, 2, 2]);
    });

    test("should handle prime numbers", () => {
      expect(factorize(17)).toEqual([17]);
      expect(factorize(19)).toEqual([19]);
      expect(factorize(23)).toEqual([23]);
    });
  });

  describe("edge cases", () => {
    test("should handle negative numbers", () => {
      expect(factorize(-12)).toEqual([2, 2, 3]);
      expect(factorize(-15)).toEqual([3, 5]);
    });

    test("should handle zero and one", () => {
      expect(factorize(0)).toEqual([]);
      expect(factorize(1)).toEqual([]);
    });

    test("should handle large numbers", () => {
      expect(factorize(997)).toEqual([997]); // Large prime
      expect(factorize(1000)).toEqual([2, 2, 2, 5, 5, 5]);
    });
  });
});
