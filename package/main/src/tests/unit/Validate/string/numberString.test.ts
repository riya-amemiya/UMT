import { numberString } from "@/Validate/string/numberString";

describe("numberString", () => {
  it("should return true for valid number strings", () => {
    const validateNumberString = numberString();
    expect(validateNumberString.validate("123")).toBe(true);
    expect(validateNumberString.validate("0.56")).toBe(true);
  });
  it("should return false for invalid number strings", () => {
    const validateNumberString = numberString();
    expect(validateNumberString.validate("abc")).toBe(false);
    expect(validateNumberString.validate("123abc")).toBe(false);
    expect(validateNumberString.validate("")).toBe(true);
  });
  it("should return the custom message for invalid input", () => {
    const customMessage = "This is not a valid number string";
    const validateNumberString = numberString(customMessage);
    expect(validateNumberString.validate("abc")).toBe(false);
    expect(validateNumberString.message).toBe(customMessage);
  });
});
