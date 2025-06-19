import { maxValue, minValue, number } from "@/Validate";
import { array } from "@/Validate/array/core";
import { maxLength, string } from "@/Validate/string";

describe("Array core validation", () => {
  // Test for empty array
  it("should validate an empty array", () => {
    const validateArray = array();
    expect(validateArray([]).validate).toBe(true);
  });

  // Test for null and undefined
  it("should fail validation for null and undefined", () => {
    const validateArray = array();
    // @ts-expect-error
    expect(validateArray(null).validate).toBe(false);
    // @ts-expect-error
    expect(validateArray(undefined).validate).toBe(false);
  });

  it("should validate an array of mixed values without type validation", () => {
    const validateArray = array();
    expect(validateArray(["apple", true, 123]).validate).toBe(true);
  });

  it("should validate an array of strings with validation", () => {
    const validateArray = array<string>({ string: string() });
    expect(validateArray(["apple", "banana", "cherry"]).validate).toBe(true);
    // @ts-expect-error
    expect(validateArray(["apple", 1, true]).validate).toBe(false);
  });

  it("should validate an array of numbers with range validation", () => {
    const validateArray = array<number>({
      number: number([minValue(0), maxValue(10)]),
    });
    expect(validateArray([1, 2, 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, 11]).validate).toBe(false);
    // @ts-expect-error
    expect(validateArray([1, "2", 3]).validate).toBe(false);
  });

  it("should validate an array of mixed types with specific validations", () => {
    const validateArray = array<number | string>({
      string: string([maxLength(1)]),
      number: number([minValue(0), maxValue(10)]),
    });
    expect(validateArray([1, 2, 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, 11]).validate).toBe(false);
    expect(validateArray([1, "2", 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, "11"]).validate).toBe(false);
  });

  it("should return a custom message on array type validation failure", () => {
    const customMessage = "Array validation failed";
    const validateArray = array<string>({ string: string() }, customMessage);
    // @ts-expect-error
    const result = validateArray("not an array");
    expect(result.validate).toBe(false);
    expect(result.message).toBe(customMessage);
  });
  it("should return element validation error message", () => {
    const elementMessage = "String too long";
    const validateArray = array<string>({
      string: string([maxLength(3, elementMessage)]),
    });
    const result = validateArray(["ok", "good", "toolong"]);
    expect(result.validate).toBe(false);
    expect(result.message).toBe(elementMessage);
  });

  it("should return an empty message when no custom message is provided", () => {
    const validateArray = array<string>({ string: string() });
    // @ts-expect-error
    const result = validateArray("not an array");
    expect(result.validate).toBe(false);
    expect(result.message).toBe("");
  });

  it("should preserve the original array in the return type", () => {
    const testArray = [1, 2, 3];
    const validateArray = array<number>();
    const result = validateArray(testArray);
    expect(result.type).toBe(testArray);
  });
});
