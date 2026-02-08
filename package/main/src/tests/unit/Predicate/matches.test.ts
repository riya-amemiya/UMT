import { matches } from "@/Predicate/matches";

describe("matches", () => {
  it("returns true when object matches the pattern", () => {
    const isAdmin = matches({ role: "admin" });
    expect(isAdmin({ name: "Alice", role: "admin" })).toBe(true);
  });

  it("returns false when object does not match", () => {
    const isAdmin = matches({ role: "admin" });
    expect(isAdmin({ name: "Bob", role: "user" })).toBe(false);
  });

  it("matches multiple properties", () => {
    const matcher = matches({ a: 1, b: 2 });
    expect(matcher({ a: 1, b: 2, c: 3 })).toBe(true);
    expect(matcher({ a: 1, b: 3 })).toBe(false);
    expect(matcher({ a: 2, b: 2 })).toBe(false);
  });

  it("uses strict equality", () => {
    const matcher = matches({ value: 0 });
    expect(matcher({ value: 0 })).toBe(true);
    expect(matcher({ value: false })).toBe(false);
    expect(matcher({ value: "" })).toBe(false);
    expect(matcher({ value: null })).toBe(false);
  });

  it("returns true for empty pattern", () => {
    const alwaysMatch = matches({});
    expect(alwaysMatch({ anything: "goes" })).toBe(true);
    expect(alwaysMatch({})).toBe(true);
  });

  it("handles missing keys in target object", () => {
    const matcher = matches({ x: 1 });
    expect(matcher({})).toBe(false);
    expect(matcher({ y: 1 })).toBe(false);
  });

  it("handles null and undefined pattern values", () => {
    const matchNull = matches({ a: null });
    expect(matchNull({ a: null })).toBe(true);
    expect(matchNull({ a: undefined })).toBe(false);

    const matchUndefined = matches({ a: undefined });
    expect(matchUndefined({ a: undefined })).toBe(true);
    expect(matchUndefined({ a: null })).toBe(false);
  });
});
