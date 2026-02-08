import { LRUCache } from "@/DataStructure/lruCache";

describe("LRUCache", () => {
  describe("constructor", () => {
    it("should create an empty cache", () => {
      const cache = new LRUCache<string, number>(3);
      expect(cache.size).toBe(0);
    });
  });

  describe("set and get", () => {
    it("should store and retrieve values", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      expect(cache.get("a")).toBe(1);
      expect(cache.get("b")).toBe(2);
      expect(cache.get("c")).toBe(3);
    });

    it("should return undefined for missing keys", () => {
      const cache = new LRUCache<string, number>(3);
      expect(cache.get("missing")).toBeUndefined();
    });

    it("should update existing key value", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("a", 10);
      expect(cache.get("a")).toBe(10);
      expect(cache.size).toBe(1);
    });
  });

  describe("eviction", () => {
    it("should evict least recently used when over capacity", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);
      cache.set("d", 4);

      expect(cache.has("a")).toBe(false);
      expect(cache.get("b")).toBe(2);
      expect(cache.get("c")).toBe(3);
      expect(cache.get("d")).toBe(4);
      expect(cache.size).toBe(3);
    });

    it("should promote accessed entries", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      cache.get("a");
      cache.set("d", 4);

      expect(cache.has("a")).toBe(true);
      expect(cache.has("b")).toBe(false);
      expect(cache.has("c")).toBe(true);
      expect(cache.has("d")).toBe(true);
    });

    it("should promote updated entries", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      cache.set("a", 10);
      cache.set("d", 4);

      expect(cache.has("a")).toBe(true);
      expect(cache.get("a")).toBe(10);
      expect(cache.has("b")).toBe(false);
    });
  });

  describe("has", () => {
    it("should return true for existing keys", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      expect(cache.has("a")).toBe(true);
    });

    it("should return false for missing keys", () => {
      const cache = new LRUCache<string, number>(3);
      expect(cache.has("a")).toBe(false);
    });
  });

  describe("delete", () => {
    it("should remove an existing entry", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);

      expect(cache.delete("a")).toBe(true);
      expect(cache.has("a")).toBe(false);
      expect(cache.size).toBe(1);
    });

    it("should return false for missing key", () => {
      const cache = new LRUCache<string, number>(3);
      expect(cache.delete("missing")).toBe(false);
    });

    it("should handle deleting head node", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      expect(cache.delete("c")).toBe(true);
      expect(cache.size).toBe(2);
      expect(cache.get("a")).toBe(1);
      expect(cache.get("b")).toBe(2);
    });

    it("should handle deleting tail node", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      expect(cache.delete("a")).toBe(true);
      expect(cache.size).toBe(2);
      expect(cache.get("b")).toBe(2);
      expect(cache.get("c")).toBe(3);
    });

    it("should handle deleting the only entry", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      expect(cache.delete("a")).toBe(true);
      expect(cache.size).toBe(0);
    });
  });

  describe("clear", () => {
    it("should remove all entries", () => {
      const cache = new LRUCache<string, number>(3);
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      cache.clear();

      expect(cache.size).toBe(0);
      expect(cache.get("a")).toBeUndefined();
      expect(cache.get("b")).toBeUndefined();
      expect(cache.get("c")).toBeUndefined();
    });
  });

  describe("size", () => {
    it("should track the number of entries", () => {
      const cache = new LRUCache<string, number>(5);
      expect(cache.size).toBe(0);

      cache.set("a", 1);
      expect(cache.size).toBe(1);

      cache.set("b", 2);
      expect(cache.size).toBe(2);

      cache.delete("a");
      expect(cache.size).toBe(1);
    });
  });

  describe("edge cases", () => {
    it("should work with capacity of 1", () => {
      const cache = new LRUCache<string, number>(1);
      cache.set("a", 1);
      expect(cache.get("a")).toBe(1);

      cache.set("b", 2);
      expect(cache.get("a")).toBeUndefined();
      expect(cache.get("b")).toBe(2);
      expect(cache.size).toBe(1);
    });

    it("should work with numeric keys", () => {
      const cache = new LRUCache<number, string>(3);
      cache.set(1, "one");
      cache.set(2, "two");
      cache.set(3, "three");

      expect(cache.get(1)).toBe("one");
      expect(cache.get(2)).toBe("two");
      expect(cache.get(3)).toBe("three");
    });

    it("should handle many operations", () => {
      const cache = new LRUCache<number, number>(100);
      for (let i = 0; i < 1000; i++) {
        cache.set(i, i * 2);
      }
      expect(cache.size).toBe(100);

      for (let i = 900; i < 1000; i++) {
        expect(cache.get(i)).toBe(i * 2);
      }

      expect(cache.get(0)).toBeUndefined();
    });

    it("should handle evict on empty cache", () => {
      const cache = new LRUCache<string, number>(3);
      // @ts-expect-error accessing private method for coverage
      cache.evict();
      expect(cache.size).toBe(0);
    });
  });
});
