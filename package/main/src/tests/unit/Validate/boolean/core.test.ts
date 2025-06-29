import { boolean } from "@/Validate/boolean/core";

describe("boolean core validation", () => {
  it("should return true for boolean values", () => {
    const validateBoolean = boolean();
    expect(validateBoolean(true).validate).toBe(true);
    expect(validateBoolean(false).validate).toBe(true);
  });

  it("should return false for non-boolean values", () => {
    const validateBoolean = boolean();
    // @ts-expect-error
    expect(validateBoolean(0).validate).toBe(false);
    // @ts-expect-error
    expect(validateBoolean(null).validate).toBe(false);
    // @ts-expect-error
    expect(validateBoolean(undefined).validate).toBe(false);
    // @ts-expect-error
    expect(validateBoolean("true").validate).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is not a boolean";
    const validateBoolean = boolean(customMessage);
    // @ts-expect-error
    expect(validateBoolean(0).validate).toBe(false);
    // @ts-expect-error
    expect(validateBoolean(0).message).toBe(customMessage);
  });
});
