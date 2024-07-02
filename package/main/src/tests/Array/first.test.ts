import { first } from "../../Array/first";

describe("first", () => {
  it("should return the first element of a non-empty array", () => {
    expect(first([1, 2, 3])).toBe(1);
    expect(first(["a", "b", "c"])).toBe("a");
    expect(first([true, false, true])).toBe(true);
    expect(first([{ key: "value" }, { key: "another" }])).toEqual({
      key: "value",
    });
  });

  it("should return undefined for an empty array", () => {
    expect(first([])).toBeUndefined();
  });

  it("should handle arrays with different types of elements", () => {
    expect(first([1, "a", true])).toBe(1);
    expect(first([undefined, null, 0])).toBeUndefined();
  });

  it("should handle arrays with only one element", () => {
    expect(first([42])).toBe(42);
    expect(first(["single"])).toBe("single");
  });
});
