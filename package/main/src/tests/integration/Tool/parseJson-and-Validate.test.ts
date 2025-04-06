import { parseJson } from "@/Tool/parseJson";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object";

/**
 * Integration tests for JSON parsing and validation
 *
 * Tests the interaction between parseJson and Validate functions to ensure:
 * - JSON strings are correctly parsed into objects
 * - Parsed objects match the expected schema
 * - Type validation works for various data types (number, boolean, string)
 * - Nested object validation works correctly
 */
describe("Integration test for 'parseJson' and 'Validate' functions", () => {
  describe("Valid JSON parsing and validation", () => {
    it("should parse JSON string with number values", () => {
      const jsonString = '{"key": 123}';
      const schema = object({
        key: number(),
      });
      const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
      const isValid = schema(result).validate;
      expect(isValid).toEqual(true);
      expect(result.key).toBe(123);
    });

    it("should parse JSON string with boolean values", () => {
      const jsonString = '{"key": true}';
      const schema = object({
        key: boolean(),
      });
      const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
      const isValid = schema(result).validate;
      expect(isValid).toEqual(true);
      expect(result.key).toBe(true);
    });

    it("should validate nested objects", () => {
      const jsonString = '{"user": {"id": 1, "active": true}}';
      const schema = object({
        user: object({
          id: number(),
          active: boolean(),
        }),
      });
      const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
      const isValid = schema(result).validate;
      expect(isValid).toEqual(true);
      expect(result.user.id).toBe(1);
      expect(result.user.active).toBe(true);
    });
  });

  describe("Invalid cases", () => {
    it("should handle invalid JSON strings", () => {
      const invalidJson = "{key: invalid}";
      expect(() => parseJson(invalidJson)).toThrow();
    });

    it("should fail validation for mismatched types", () => {
      const jsonString = '{"key": "123"}'; // string instead of number
      const schema = object({
        key: number(),
      });
      const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
      const isValid = schema(result).validate;
      expect(isValid).toEqual(false);
    });

    it("should fail validation for missing required fields", () => {
      const jsonString = '{"id": 1}'; // missing 'active' field
      const schema = object({
        id: number(),
        active: boolean(),
      });
      const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
      const isValid = schema(result).validate;
      expect(isValid).toEqual(false);
    });
  });
});
