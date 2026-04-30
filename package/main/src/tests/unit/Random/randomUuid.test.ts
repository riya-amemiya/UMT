import { randomUUID } from "@/Random/randomUuid";

describe("randomUUID", () => {
  const UUID_V4_PATTERN =
    /^[\da-f]{8}-[\da-f]{4}-4[\da-f]{3}-[89ab][\da-f]{3}-[\da-f]{12}$/i;

  it("returns a v4-formatted UUID", () => {
    expect(randomUUID()).toMatch(UUID_V4_PATTERN);
  });

  it("returns unique values across calls", () => {
    const set = new Set<string>();
    for (let index = 0; index < 100; index += 1) {
      set.add(randomUUID());
    }
    expect(set.size).toBe(100);
  });

  it("falls back to getRandomValues when crypto.randomUUID is unavailable", () => {
    const originalRandomUUID = globalThis.crypto.randomUUID;
    Object.defineProperty(globalThis.crypto, "randomUUID", {
      value: undefined,
      configurable: true,
    });
    try {
      const value = randomUUID();
      expect(value).toMatch(UUID_V4_PATTERN);
    } finally {
      Object.defineProperty(globalThis.crypto, "randomUUID", {
        value: originalRandomUUID,
        configurable: true,
      });
    }
  });
});
