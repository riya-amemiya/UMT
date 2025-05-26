import { compareFunctionDefault } from "@/Array/compareFunctionDefault";

describe("compareFunctionDefault", () => {
  it("should return 0 for equal numbers", () => {
    expect(compareFunctionDefault(1, 1)).toBe(0);
  });

  it("should return 1 if a > b", () => {
    expect(compareFunctionDefault(2, 1)).toBe(1);
  });

  it("should return -1 if a < b", () => {
    expect(compareFunctionDefault(1, 2)).toBe(-1);
  });

  it("should return 0 for equal strings", () => {
    expect(compareFunctionDefault("a", "a")).toBe(0);
  });

  it("should return 1 if a > b for strings", () => {
    expect(compareFunctionDefault("b", "a")).toBe(1);
  });

  it("should return -1 if a < b for strings", () => {
    expect(compareFunctionDefault("a", "b")).toBe(-1);
  });

  // Coverage for undefined and null cases
  it("should return 0 if both are undefined", () => {
    expect(compareFunctionDefault(undefined, undefined)).toBe(0);
  });

  it("should return 1 if a is undefined and b is defined", () => {
    expect(compareFunctionDefault(undefined, 1)).toBe(1);
  });

  it("should return -1 if b is undefined and a is defined", () => {
    expect(compareFunctionDefault(1, undefined)).toBe(-1);
  });

  it("should return 0 if both are null", () => {
    expect(compareFunctionDefault(null, null)).toBe(0);
  });

  it("should return 1 if a is null and b is not", () => {
    expect(compareFunctionDefault(null, 1)).toBe(1);
  });

  it("should return -1 if b is null and a is not", () => {
    expect(compareFunctionDefault(1, null)).toBe(-1);
  });

  it("should handle mixed undefined and null", () => {
    expect(compareFunctionDefault(undefined, null)).toBe(1); // undefined > null
    expect(compareFunctionDefault(null, undefined)).toBe(-1); // null < undefined
  });
});
