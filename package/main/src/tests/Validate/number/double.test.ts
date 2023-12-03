import { double } from "@/Validate/number/double";

describe("double", () => {
  it("should return true for double numbers", () => {
    const validateDouble = double();
    expect(validateDouble.validate(1.5)).toBe(true);
    expect(validateDouble.validate(2.22)).toBe(true);
    expect(validateDouble.validate(0.1)).toBe(true);
  });

  it("should return false for non-double numbers", () => {
    const validateDouble = double();
    expect(validateDouble.validate(1)).toBe(false);
    expect(validateDouble.validate(100)).toBe(false);
    expect(validateDouble.validate(0)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is not a double number";
    const validateDouble = double(customMessage);
    expect(validateDouble.validate(1)).toBe(false);
    expect(validateDouble.message).toBe(customMessage);
  });
});
