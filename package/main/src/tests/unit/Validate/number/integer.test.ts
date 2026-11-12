import { number } from "@/Validate/number";
import { integer } from "@/Validate/number/integer";

describe("integer", () => {
  it("should return true for integers", () => {
    const validateInteger = integer();
    expect(validateInteger.validate(0)).toBe(true);
    expect(validateInteger.validate(1)).toBe(true);
    expect(validateInteger.validate(-1)).toBe(true);
    expect(validateInteger.validate(1.0)).toBe(true);
    expect(validateInteger.validate(Number.MAX_SAFE_INTEGER)).toBe(true);
    expect(validateInteger.validate(1e21)).toBe(true);
  });

  it("should return false for non-integers", () => {
    const validateInteger = integer();
    expect(validateInteger.validate(1.5)).toBe(false);
    expect(validateInteger.validate(-0.1)).toBe(false);
    expect(validateInteger.validate(Number.NaN)).toBe(false);
    expect(validateInteger.validate(Number.POSITIVE_INFINITY)).toBe(false);
    expect(validateInteger.validate(Number.NEGATIVE_INFINITY)).toBe(false);
  });

  it("should expose the custom message and the number type", () => {
    const validateInteger = integer("must be an integer");
    expect(validateInteger.message).toBe("must be an integer");
    expect(validateInteger.type).toBe("number");
    expect(integer().message).toBeUndefined();
  });

  it("should compose with number()", () => {
    const validator = number([integer("must be an integer")]);
    expect(validator(3).validate).toBe(true);
    expect(validator(3.5)).toEqual({
      validate: false,
      message: "must be an integer",
      type: "number",
    });
  });
});
