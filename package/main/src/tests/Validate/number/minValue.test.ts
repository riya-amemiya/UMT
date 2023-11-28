import { minValue } from "@/Validate/number/minValue";

describe("minValue", () => {
  it("should return true for numbers greater than or equal to the minimum value", () => {
    const validateMinValue = minValue(10);
    expect(validateMinValue.validate(10)).toBe(true);
    expect(validateMinValue.validate(15)).toBe(true);
  });

  it("should return false for numbers less than the minimum value", () => {
    const validateMinValue = minValue(10);
    expect(validateMinValue.validate(9)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "Value must be at least 10";
    const validateMinValue = minValue(10, customMessage);
    expect(validateMinValue.validate(9)).toBe(false);
    expect(validateMinValue.message).toBe(customMessage);
  });
});
