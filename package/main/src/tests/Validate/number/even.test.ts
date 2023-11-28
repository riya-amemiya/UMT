import { even } from "@/Validate/number/even";

describe("even", () => {
  it("should return true for even numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(2)).toBe(true);
    expect(validateEven.validate(4)).toBe(true);
    expect(validateEven.validate(6)).toBe(true);
  });

  it("should return false for odd numbers", () => {
    const validateEven = even();
    expect(validateEven.validate(1)).toBe(false);
    expect(validateEven.validate(3)).toBe(false);
    expect(validateEven.validate(5)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is a custom message";
    const validateEven = even(customMessage);
    expect(validateEven.validate(1)).toBe(false);
    expect(validateEven.message).toBe(customMessage);
  });
});
