import { number } from "@/Validate/number";
import { between } from "@/Validate/number/between";

describe("between", () => {
  it("should accept values inside the range including both bounds", () => {
    const validateBetween = between(1, 10);
    expect(validateBetween.validate(1)).toBe(true);
    expect(validateBetween.validate(5.5)).toBe(true);
    expect(validateBetween.validate(10)).toBe(true);
  });

  it("should reject values below the minimum", () => {
    const validateBetween = between(1, 10);
    expect(validateBetween.validate(0.999)).toBe(false);
    expect(validateBetween.validate(-1)).toBe(false);
  });

  it("should reject values above the maximum", () => {
    const validateBetween = between(1, 10);
    expect(validateBetween.validate(10.001)).toBe(false);
    expect(validateBetween.validate(100)).toBe(false);
  });

  it("should reject NaN", () => {
    expect(between(1, 10).validate(Number.NaN)).toBe(false);
  });

  it("should accept only the bound when min equals max", () => {
    const validateBetween = between(3, 3);
    expect(validateBetween.validate(3)).toBe(true);
    expect(validateBetween.validate(2)).toBe(false);
    expect(validateBetween.validate(4)).toBe(false);
  });

  it("should reject every value when min is greater than max", () => {
    const validateBetween = between(10, 1);
    expect(validateBetween.validate(5)).toBe(false);
    expect(validateBetween.validate(1)).toBe(false);
    expect(validateBetween.validate(10)).toBe(false);
  });

  it("should report one message for both bounds through number()", () => {
    const validator = number([between(1, 10, "must be between 1 and 10")]);
    expect(validator(5).validate).toBe(true);
    expect(validator(0).message).toBe("must be between 1 and 10");
    expect(validator(11).message).toBe("must be between 1 and 10");
    expect(between(1, 10).message).toBeUndefined();
    expect(between(1, 10).type).toBe("number");
  });
});
