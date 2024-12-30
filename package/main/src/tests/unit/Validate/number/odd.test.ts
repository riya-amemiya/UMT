import { odd } from "@/Validate/number/odd";

describe("odd", () => {
  it("should return true for odd numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(1)).toBe(true);
    expect(validateOdd.validate(3)).toBe(true);
    expect(validateOdd.validate(5)).toBe(true);
  });

  it("should return false for even numbers", () => {
    const validateOdd = odd();
    expect(validateOdd.validate(2)).toBe(false);
    expect(validateOdd.validate(4)).toBe(false);
    expect(validateOdd.validate(6)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is a custom message";
    const validateOdd = odd(customMessage);
    expect(validateOdd.validate(2)).toBe(false);
    expect(validateOdd.message).toBe(customMessage);
  });
});
