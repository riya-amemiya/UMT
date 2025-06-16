import { odd } from "@/Validate/number/odd";

describe("odd", () => {
  it("should return true for odd numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(1)).toBe(true);
    expect(validateOdd.validate(3)).toBe(true);
    expect(validateOdd.validate(5)).toBe(true);
    expect(validateOdd.validate(7)).toBe(true);
    expect(validateOdd.validate(9)).toBe(true);
    expect(validateOdd.validate(99)).toBe(true);
    expect(validateOdd.validate(999)).toBe(true);
  });

  it("should return false for even numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(2)).toBe(false);
    expect(validateOdd.validate(4)).toBe(false);
    expect(validateOdd.validate(6)).toBe(false);
    expect(validateOdd.validate(8)).toBe(false);
    expect(validateOdd.validate(10)).toBe(false);
    expect(validateOdd.validate(100)).toBe(false);
    expect(validateOdd.validate(1000)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is a custom message";
    const validateOdd = odd(customMessage);
    expect(validateOdd.validate(2)).toBe(false);
    expect(validateOdd.message).toBe(customMessage);
  });

  it("should handle zero", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(0)).toBe(false);
  });

  it("should handle negative numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(-1)).toBe(true);
    expect(validateOdd.validate(-3)).toBe(true);
    expect(validateOdd.validate(-5)).toBe(true);
    expect(validateOdd.validate(-2)).toBe(false);
    expect(validateOdd.validate(-4)).toBe(false);
    expect(validateOdd.validate(-6)).toBe(false);
  });

  it("should handle decimal numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(1.5)).toBe(false);
    expect(validateOdd.validate(3.7)).toBe(false);
    expect(validateOdd.validate(2.3)).toBe(false);
    expect(validateOdd.validate(4.9)).toBe(false);
  });

  it("should handle large numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(999999)).toBe(true);
    expect(validateOdd.validate(1000000)).toBe(false);
    expect(validateOdd.validate(Number.MAX_SAFE_INTEGER)).toBe(true);
    expect(validateOdd.validate(Number.MAX_SAFE_INTEGER - 1)).toBe(false);
  });
});
