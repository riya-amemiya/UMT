import {
  afterEach,
  beforeEach,
  describe,
  expect,
  it,
  jest,
} from "@jest/globals";
import { TTLCache } from "@/DataStructure/ttlCache";

describe("TTLCache", () => {
  beforeEach(() => {
    jest.useFakeTimers();
    jest.spyOn(Date, "now");
  });

  afterEach(() => {
    jest.useRealTimers();
    jest.restoreAllMocks();
  });

  describe("constructor", () => {
    it("should create an empty cache", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      expect(cache.size).toBe(0);
    });
  });

  describe("set and get", () => {
    it("should store and retrieve values", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);
      cache.set("b", 2);

      expect(cache.get("a")).toBe(1);
      expect(cache.get("b")).toBe(2);
    });

    it("should return undefined for missing keys", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      expect(cache.get("missing")).toBeUndefined();
    });

    it("should update existing key value", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);
      cache.set("a", 10);
      expect(cache.get("a")).toBe(10);
      expect(cache.size).toBe(1);
    });
  });

  describe("TTL expiration", () => {
    it("should expire entries after defaultTTL", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);

      expect(cache.get("a")).toBe(1);

      jest.advanceTimersByTime(5000);

      expect(cache.get("a")).toBeUndefined();
    });

    it("should not expire entries before TTL", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);

      jest.advanceTimersByTime(4999);

      expect(cache.get("a")).toBe(1);
    });

    it("should support per-entry TTL override", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("short", 1, 1000);
      cache.set("long", 2, 10_000);

      jest.advanceTimersByTime(1000);

      expect(cache.get("short")).toBeUndefined();
      expect(cache.get("long")).toBe(2);

      jest.advanceTimersByTime(9000);

      expect(cache.get("long")).toBeUndefined();
    });

    it("should expire entries checked via has()", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);

      expect(cache.has("a")).toBe(true);

      jest.advanceTimersByTime(5000);

      expect(cache.has("a")).toBe(false);
    });

    it("should clean up expired entries on get()", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);

      jest.advanceTimersByTime(5000);

      expect(cache.size).toBe(1);
      cache.get("a");
      expect(cache.size).toBe(0);
    });

    it("should clean up expired entries on has()", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);

      jest.advanceTimersByTime(5000);

      expect(cache.size).toBe(1);
      cache.has("a");
      expect(cache.size).toBe(0);
    });
  });

  describe("maxSize", () => {
    it("should evict oldest entry when maxSize exceeded", () => {
      const cache = new TTLCache<string, number>({
        defaultTTL: 5000,
        maxSize: 2,
      });
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      expect(cache.size).toBe(2);
      expect(cache.get("a")).toBeUndefined();
      expect(cache.get("b")).toBe(2);
      expect(cache.get("c")).toBe(3);
    });

    it("should not evict when updating existing key", () => {
      const cache = new TTLCache<string, number>({
        defaultTTL: 5000,
        maxSize: 2,
      });
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("a", 10);

      expect(cache.size).toBe(2);
      expect(cache.get("a")).toBe(10);
      expect(cache.get("b")).toBe(2);
    });
  });

  describe("has", () => {
    it("should return true for existing non-expired keys", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);
      expect(cache.has("a")).toBe(true);
    });

    it("should return false for missing keys", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      expect(cache.has("a")).toBe(false);
    });
  });

  describe("delete", () => {
    it("should remove an existing entry", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);
      expect(cache.delete("a")).toBe(true);
      expect(cache.has("a")).toBe(false);
      expect(cache.size).toBe(0);
    });

    it("should return false for missing key", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      expect(cache.delete("missing")).toBe(false);
    });
  });

  describe("clear", () => {
    it("should remove all entries", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 5000 });
      cache.set("a", 1);
      cache.set("b", 2);
      cache.set("c", 3);

      cache.clear();

      expect(cache.size).toBe(0);
      expect(cache.get("a")).toBeUndefined();
    });
  });

  describe("edge cases", () => {
    it("should work with maxSize of 1", () => {
      const cache = new TTLCache<string, number>({
        defaultTTL: 5000,
        maxSize: 1,
      });
      cache.set("a", 1);
      cache.set("b", 2);

      expect(cache.size).toBe(1);
      expect(cache.get("a")).toBeUndefined();
      expect(cache.get("b")).toBe(2);
    });

    it("should handle TTL of 0 (immediate expiration)", () => {
      const cache = new TTLCache<string, number>({ defaultTTL: 0 });
      cache.set("a", 1);

      expect(cache.get("a")).toBeUndefined();
    });

    it("should handle numeric keys", () => {
      const cache = new TTLCache<number, string>({ defaultTTL: 5000 });
      cache.set(1, "one");
      cache.set(2, "two");

      expect(cache.get(1)).toBe("one");
      expect(cache.get(2)).toBe("two");
    });
  });
});
