import { maxValue, minValue, number } from "@/Validate";
import { array } from "@/Validate/array/core";
import { maxLength, string } from "@/Validate/string";

describe("array core validation", () => {
  it("should validate an array of strings", () => {
    const validateArray = array<string>({ string: string() });
    expect(validateArray(["apple", "banana", "cherry"]).validate).toBe(true);
    // @ts-ignore
    expect(validateArray(["apple", 1, true]).validate).toBe(false);
  });

  it("should validate an array of numbers", () => {
    const validateArray = array<number>({
      number: number([minValue(0), maxValue(10)]),
    });
    expect(validateArray([1, 2, 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, 11]).validate).toBe(false);
    // @ts-ignore
    expect(validateArray([1, "2", 3]).validate).toBe(false);
  });

  it("should validate an array of numbers and strings", () => {
    const validateArray = array<number | string>({
      string: string([maxLength(1)]),
      number: number([minValue(0), maxValue(10)]),
    });
    expect(validateArray([1, 2, 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, 11]).validate).toBe(false);
    expect(validateArray([1, "2", 3]).validate).toBe(true);
    expect(validateArray([1, 2, 3, "11"]).validate).toBe(false);
  });

  it("should return a custom message on validation failure", () => {
    const customMessage = "Array validation failed";
    const validateArray = array<string>({ string: string() }, customMessage);
    // @ts-ignore
    const result = validateArray(1);
    expect(result.validate).toBe(false);
    expect(result.message).toBe(customMessage);
  });
  it("should return an empty message on validation failure", () => {
    const validateArray = array<string>({ string: string() });
    // @ts-ignore
    const result = validateArray(1);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("");
  });
});
