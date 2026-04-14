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
  });

  describe("BloomFilter.fromExpected", () => {
    it("should create a filter with optimal parameters", () => {
      const filter = BloomFilter.fromExpected(1000, 0.01);
      expect(filter.bitSize).toBeGreaterThan(0);
      expect(filter.numHashFunctions).toBeGreaterThanOrEqual(1);
    });

    it("should produce a larger bit array for a stricter false-positive rate", () => {
      const strict = BloomFilter.fromExpected(1000, 0.001);
      const loose = BloomFilter.fromExpected(1000, 0.1);
      expect(strict.bitSize).toBeGreaterThan(loose.bitSize);
    });
  });

  describe("add and has", () => {
    it("should return true for a single added item", () => {
      const filter = new BloomFilter();
      filter.add("hello");
      expect(filter.has("hello")).toBe(true);
    });

    it("should return false for an item that was never added", () => {
      const filter = new BloomFilter({ size: 10_000, hashCount: 7 });
      expect(filter.has("not-added")).toBe(false);
    });

    it("should never produce false negatives for added items", () => {
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
      filter.add("a".repeat(10_000));
      expect(filter.has("a".repeat(10_000))).toBe(true);
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
      expect(filter.estimatedFalsePositiveRate(500)).toBeGreaterThan(
        filter.estimatedFalsePositiveRate(100),
      );
    });

    it("should be close to the target rate when using fromExpected", () => {
      const filter = BloomFilter.fromExpected(1000, 0.01);
      expect(filter.estimatedFalsePositiveRate(1000)).toBeLessThan(0.011);
    });
  });

  describe("false positive rate in practice", () => {
    it("should stay below 5% for a 1% target after inserting 1000 items", () => {
      const filter = BloomFilter.fromExpected(1000, 0.01);
      for (let i = 0; i < 1000; i++) {
        filter.add(`item-${i}`);
      }
      let falsePositives = 0;
      for (let i = 1000; i < 2000; i++) {
        if (filter.has(`item-${i}`)) {
          falsePositives++;
        }
      }
      expect(falsePositives / 1000).toBeLessThan(0.05);
    });
  });
});
