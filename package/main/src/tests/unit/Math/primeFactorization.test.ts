import { primeFactorization } from "@/Math/primeFactorization";

describe("primeFactorization function", () => {
  describe("basic prime factorization", () => {
    it("should factorize numbers with single prime factor", () => {
      expect(primeFactorization(8)).toEqual([{ number: 2, count: 3 }]);
      expect(primeFactorization(27)).toEqual([{ number: 3, count: 3 }]);
    });

    it("should factorize numbers with multiple prime factors", () => {
      expect(primeFactorization(12)).toEqual([
        { number: 2, count: 2 },
        { number: 3, count: 1 },
      ]);
      expect(primeFactorization(100)).toEqual([
        { number: 2, count: 2 },
        { number: 5, count: 2 },
      ]);
    });

    it("should handle prime numbers", () => {
      expect(primeFactorization(2)).toEqual([{ number: 2, count: 1 }]);
      expect(primeFactorization(17)).toEqual([{ number: 17, count: 1 }]);
      expect(primeFactorization(31)).toEqual([{ number: 31, count: 1 }]);
    });
  });

  describe("edge cases", () => {
    it("should handle perfect squares", () => {
      expect(primeFactorization(25)).toEqual([{ number: 5, count: 2 }]);
      expect(primeFactorization(49)).toEqual([{ number: 7, count: 2 }]);
    });

    it("should handle numbers with large prime factors", () => {
      expect(primeFactorization(997)).toEqual([{ number: 997, count: 1 }]);
      expect(primeFactorization(998)).toEqual([
        { number: 2, count: 1 },
        { number: 499, count: 1 },
      ]);
    });

    it("should handle consecutive prime factors", () => {
      expect(primeFactorization(30)).toEqual([
        { number: 2, count: 1 },
        { number: 3, count: 1 },
        { number: 5, count: 1 },
      ]);
    });
  });
});
