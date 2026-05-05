import { nullType } from "@/Validate/null/core";

describe("null core validation", () => {
  it("should return true for null values", () => {
    const validateNull = nullType();
    expect(validateNull(null).validate).toBe(true);
    expect(validateNull(null).type).toBe("null");
  });

  it("should return false for non-null values", () => {
    const validateNull = nullType();
    // @ts-expect-error
    expect(validateNull(undefined).validate).toBe(false);
    // @ts-expect-error
    expect(validateNull(0).validate).toBe(false);
    // @ts-expect-error
    expect(validateNull("").validate).toBe(false);
    // @ts-expect-error
    expect(validateNull(false).validate).toBe(false);
    // @ts-expect-error
    expect(validateNull({}).validate).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is not null";
    const validateNull = nullType(customMessage);
    // @ts-expect-error
    expect(validateNull(undefined).validate).toBe(false);
    // @ts-expect-error
    expect(validateNull(undefined).message).toBe(customMessage);
  });

  it("should return empty message for valid null input", () => {
    const validateNull = nullType("custom message");
    expect(validateNull(null).validate).toBe(true);
    expect(validateNull(null).message).toBe("");
  });
});
