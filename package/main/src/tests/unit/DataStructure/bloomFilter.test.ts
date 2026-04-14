import { BloomFilter } from "@/DataStructure/bloomFilter";

describe("BloomFilter", () => {
  describe("constructor", () => {
    it("should create a filter with default parameters", () => {
      const filter = new BloomFilter();
      expect(filter.bitSize).toBe(1000);
      expect(filter.numHashFunctions).toBe(7);
    });

    it("should create a filter with explicit parameters", () => {
      const filter = new BloomFilter({ size: 2048, hashCount: 5 });
      expect(filter.bitSize).toBe(2048);
      expect(filter.numHashFunctions).toBe(5);
    });

    it("should throw when size is less than 1", () => {
      expect(() => new BloomFilter({ size: 0 })).toThrow(
        "BloomFilter size must be at least 1",
      );
    });

    it("should throw when hashCount is less than 1", () => {
      expect(() => new BloomFilter({ hashCount: 0 })).toThrow(
        "BloomFilter hashCount must be at least 1",
      );
    });
  });

  describe("BloomFilter.fromExpected", () => {
    it("should create a filter with optimal parameters", () => {
      const filter = BloomFilter.fromExpected(1000, 0.01);
      expect(filter.bitSize).toBeGreaterThan(0);
      expect(filter.numHashFunctions).toBeGreaterThanOrEqual(1);
    });

    it("should produce a lower false-positive rate for larger size", () => {
      const strict = BloomFilter.fromExpected(1000, 0.001);
      const loose = BloomFilter.fromExpected(1000, 0.1);
      expect(strict.bitSize).toBeGreaterThan(loose.bitSize);
    });

    it("should throw when expectedItems is less than 1", () => {
      expect(() => BloomFilter.fromExpected(0, 0.01)).toThrow(
        "expectedItems must be at least 1",
      );
    });

    it("should throw when falsePositiveRate is 0", () => {
      expect(() => BloomFilter.fromExpected(100, 0)).toThrow(
        "falsePositiveRate must be in the range (0, 1)",
      );
    });

    it("should throw when falsePositiveRate is 1", () => {
      expect(() => BloomFilter.fromExpected(100, 1)).toThrow(
        "falsePositiveRate must be in the range (0, 1)",
      );
    });
  });

  describe("add and has", () => {
    it("should return true for an added item", () => {
      const filter = new BloomFilter();
      filter.add("hello");
      expect(filter.has("hello")).toBe(true);
    });

    it("should return false for an item that was never added", () => {
      const filter = new BloomFilter({ size: 10000, hashCount: 7 });
      expect(filter.has("not-added")).toBe(false);
    });

    it("should handle multiple items without false negatives", () => {
      const filter = BloomFilter.fromExpected(100, 0.01);
      const items = ["apple", "banana", "cherry", "date", "elderberry"];
      for (const item of items) {
        filter.add(item);
      }
      for (const item of items) {
        expect(filter.has(item)).toBe(true);
      }
    });

    it("should be case-sensitive", () => {
      const filter = new BloomFilter();
      filter.add("Hello");
      expect(filter.has("Hello")).toBe(true);
      expect(filter.has("hello")).toBe(false);
    });

    it("should handle empty string", () => {
      const filter = new BloomFilter();
      filter.add("");
      expect(filter.has("")).toBe(true);
    });

    it("should handle unicode strings", () => {
      const filter = new BloomFilter();
      filter.add("こんにちは");
      expect(filter.has("こんにちは")).toBe(true);
      expect(filter.has("さようなら")).toBe(false);
    });

    it("should handle long strings", () => {
      const filter = new BloomFilter();
      const long = "a".repeat(10000);
      filter.add(long);
      expect(filter.has(long)).toBe(true);
      expect(filter.has("a".repeat(9999))).toBe(false);
    });
  });

  describe("clear", () => {
    it("should make previously added items not found", () => {
      const filter = new BloomFilter();
      filter.add("hello");
      filter.add("world");
      filter.clear();
      expect(filter.has("hello")).toBe(false);
      expect(filter.has("world")).toBe(false);
    });

    it("should allow re-adding items after clear", () => {
      const filter = new BloomFilter();
      filter.add("hello");
      filter.clear();
      filter.add("hello");
      expect(filter.has("hello")).toBe(true);
    });
  });

  describe("estimatedFalsePositiveRate", () => {
    it("should return 0 for 0 inserted items", () => {
      const filter = new BloomFilter({ size: 1000, hashCount: 7 });
      expect(filter.estimatedFalsePositiveRate(0)).toBe(0);
    });

    it("should increase as more items are inserted", () => {
      const filter = new BloomFilter({ size: 1000, hashCount: 7 });
      const rate100 = filter.estimatedFalsePositiveRate(100);
      const rate500 = filter.estimatedFalsePositiveRate(500);
      expect(rate500).toBeGreaterThan(rate100);
    });

    it("should be close to the target rate when using fromExpected", () => {
      const targetRate = 0.01;
      const filter = BloomFilter.fromExpected(1000, targetRate);
      const actual = filter.estimatedFalsePositiveRate(1000);
      // Allow a small tolerance due to rounding of optimal parameters
      expect(actual).toBeLessThan(targetRate * 1.1);
    });
  });

  describe("false positive rate in practice", () => {
    it("should have a low false positive rate", () => {
      const n = 1000;
      const targetFpr = 0.01;
      const filter = BloomFilter.fromExpected(n, targetFpr);

      // Insert n items
      for (let i = 0; i < n; i++) {
        filter.add(`item-${i}`);
      }

      // Test n unseen items and count false positives
      let falsePositives = 0;
      const testCount = 1000;
      for (let i = n; i < n + testCount; i++) {
        if (filter.has(`item-${i}`)) {
          falsePositives++;
        }
      }

      const observedFpr = falsePositives / testCount;
      // Generous bound: observed FPR should be under 5% for a 1% target
      expect(observedFpr).toBeLessThan(0.05);
    });
  });
});
