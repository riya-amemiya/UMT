import { maxValue } from "@/Validate/number/maxValue";

describe("maxValue", () => {
  it("should return true for numbers less than or equal to the max value", () => {
    const validateMaxValue = maxValue(10);
    expect(validateMaxValue.validate(5)).toBe(true);
    expect(validateMaxValue.validate(10)).toBe(true);
  });

  it("should return false for numbers greater than the max value", () => {
    const validateMaxValue = maxValue(10);
    expect(validateMaxValue.validate(11)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "Value must be less than or equal to 10";
    const validateMaxValue = maxValue(10, customMessage);
    expect(validateMaxValue.validate(11)).toBe(false);
    expect(validateMaxValue.message).toBe(customMessage);
  });
});
