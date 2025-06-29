import { core } from "@/Validate/core";

import type { ValidateReturnType } from "@/Validate/type";

describe("Core validation", () => {
  it("should validate type correctly", () => {
    const validateString = core("string");
    expect(validateString("test").validate).toBe(true);
    // @ts-expect-error
    expect(validateString(123).validate).toBe(false);
  });

  it("should respect custom message for type validation", () => {
    const message = "Not a string";
    const validateString = core("string");
    // @ts-expect-error
    expect(validateString(123, [], message).message).toBe(message);
  });

  it("should return empty message when no message provided", () => {
    const validateString = core("string");
    // @ts-expect-error
    expect(validateString(123).message).toBe("");
  });

  it("should validate with additional validation rules", () => {
    const validateString = core("string");
    const minLength: ValidateReturnType<string> = {
      type: "string",
      validate: (value: string) => value.length >= 3,
      message: "String too short",
    };

    expect(validateString("test", [minLength]).validate).toBe(true);
    expect(validateString("ab", [minLength]).validate).toBe(false);
    expect(validateString("ab", [minLength]).message).toBe("String too short");

    // Test validation rule without message
    const noMessage: ValidateReturnType<string> = {
      type: "string",
      validate: (value: string) => value.length >= 3,
    };
    expect(validateString("ab", [noMessage]).message).toBe("");
  });

  it("should validate numbers", () => {
    const validateNumber = core<number>("number");
    expect(validateNumber(123).validate).toBe(true);
    // @ts-expect-error
    expect(validateNumber("123").validate).toBe(false);
  });

  it("should validate booleans", () => {
    const validateBoolean = core<boolean>("boolean");
    expect(validateBoolean(true).validate).toBe(true);
    expect(validateBoolean(false).validate).toBe(true);
    // @ts-expect-error
    expect(validateBoolean("true").validate).toBe(false);
  });

  it("should validate with multiple validation rules", () => {
    const validateString = core("string");
    const minLength: ValidateReturnType<string> = {
      type: "string",
      validate: (value: string) => value.length >= 3,
      message: "String too short",
    };
    const maxLength: ValidateReturnType<string> = {
      type: "string",
      validate: (value: string) => value.length <= 10,
      message: "String too long",
    };

    expect(validateString("test", [minLength, maxLength]).validate).toBe(true);
    expect(validateString("ab", [minLength, maxLength]).validate).toBe(false);
    expect(validateString("ab", [minLength, maxLength]).message).toBe(
      "String too short",
    );
    expect(
      validateString("very long string", [minLength, maxLength]).validate,
    ).toBe(false);
    expect(
      validateString("very long string", [minLength, maxLength]).message,
    ).toBe("String too long");
  });

  it("should preserve type information in return value", () => {
    const validateString = core<string>("string");
    const validateNumber = core<number>("number");
    const validateBoolean = core<boolean>("boolean");

    expect(validateString("test").type).toBe("string");
    expect(validateNumber(123).type).toBe("number");
    expect(validateBoolean(true).type).toBe("boolean");
  });
});
