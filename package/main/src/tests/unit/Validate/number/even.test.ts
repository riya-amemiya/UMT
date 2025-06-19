import { even } from "@/Validate/number/even";

describe("even", () => {
  it("should return true for even numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(2)).toBe(true);
    expect(validateEven.validate(4)).toBe(true);
    expect(validateEven.validate(6)).toBe(true);
    expect(validateEven.validate(8)).toBe(true);
    expect(validateEven.validate(10)).toBe(true);
    expect(validateEven.validate(100)).toBe(true);
    expect(validateEven.validate(1000)).toBe(true);
  });

  it("should return false for odd numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(1)).toBe(false);
    expect(validateEven.validate(3)).toBe(false);
    expect(validateEven.validate(5)).toBe(false);
    expect(validateEven.validate(7)).toBe(false);
    expect(validateEven.validate(9)).toBe(false);
    expect(validateEven.validate(99)).toBe(false);
    expect(validateEven.validate(999)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is a custom message";
    const validateEven = even(customMessage);
    expect(validateEven.validate(1)).toBe(false);
    expect(validateEven.message).toBe(customMessage);
  });

  it("should handle zero", () => {
    const validateEven = even();
    expect(validateEven.validate(0)).toBe(true);
  });

  it("should handle negative numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(-2)).toBe(true);
    expect(validateEven.validate(-4)).toBe(true);
    expect(validateEven.validate(-6)).toBe(true);
    expect(validateEven.validate(-1)).toBe(false);
    expect(validateEven.validate(-3)).toBe(false);
    expect(validateEven.validate(-5)).toBe(false);
  });

  it("should handle decimal numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(2.5)).toBe(false);
    expect(validateEven.validate(4.7)).toBe(false);
    expect(validateEven.validate(1.3)).toBe(false);
    expect(validateEven.validate(3.9)).toBe(false);
  });

  it("should handle large numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(1_000_000)).toBe(true);
    expect(validateEven.validate(999_999)).toBe(false);
    expect(validateEven.validate(Number.MAX_SAFE_INTEGER - 1)).toBe(true);
    expect(validateEven.validate(Number.MAX_SAFE_INTEGER)).toBe(false);
  });
});
