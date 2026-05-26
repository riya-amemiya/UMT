import { zipToMap } from "@/Map/zipToMap";

describe("zipToMap", () => {
  it("should pair keys and values at the same index", () => {
    const result = zipToMap(["a", "b", "c"], [1, 2, 3]);
    expect(result).toEqual(
      new Map<string, number>([
        ["a", 1],
        ["b", 2],
        ["c", 3],
      ]),
    );
  });

  it("should stop at the shorter length when keys is longer", () => {
    const result = zipToMap(["a", "b", "c"], [1, 2]);
    expect(result).toEqual(
      new Map<string, number>([
        ["a", 1],
        ["b", 2],
      ]),
    );
  });

  it("should stop at the shorter length when values is longer", () => {
    const result = zipToMap(["a", "b"], [1, 2, 3]);
    expect(result).toEqual(
      new Map<string, number>([
        ["a", 1],
        ["b", 2],
      ]),
    );
  });

  it("should keep the latest value for duplicate keys", () => {
    const result = zipToMap(["a", "b", "a"], [1, 2, 3]);
    expect(result.get("a")).toBe(3);
    expect(result.get("b")).toBe(2);
    expect(result.size).toBe(2);
  });

  it("should accept object keys", () => {
    const k1 = { id: 1 };
    const k2 = { id: 2 };
    const result = zipToMap([k1, k2], ["x", "y"]);
    expect(result.get(k1)).toBe("x");
    expect(result.get(k2)).toBe("y");
  });

  it("should return an empty Map when either input is empty", () => {
    expect(zipToMap([], [1, 2]).size).toBe(0);
    expect(zipToMap(["a"], []).size).toBe(0);
  });

  it("should preserve insertion order of keys", () => {
    const result = zipToMap(["c", "a", "b"], [1, 2, 3]);
    expect([...result.keys()]).toEqual(["c", "a", "b"]);
  });
});
