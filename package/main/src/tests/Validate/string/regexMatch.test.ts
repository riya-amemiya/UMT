import { regexMatch } from "@/Validate/string/regexMatch";

describe("regexMatch", () => {
  test("should validate matching patterns", () => {
    const pattern = /^[a-z]+$/;
    const validator = regexMatch(pattern);
    expect(validator.validate("abc")).toBe(true);
    expect(validator.validate("ABC")).toBe(false);
    expect(validator.validate("123")).toBe(false);
    expect(validator.validate("abc123")).toBe(false);
  });

  test("should validate non-matching patterns", () => {
    const pattern = /^[0-9]+$/;
    const validator = regexMatch(pattern);
    expect(validator.validate("123")).toBe(true);
    expect(validator.validate("abc")).toBe(false);
    expect(validator.validate("123abc")).toBe(false);
    expect(validator.validate("")).toBe(false);
  });

  test("should handle custom error messages", () => {
    const pattern = /^[A-Z]+$/;
    const message = "Only uppercase letters are allowed";
    const validator = regexMatch(pattern, message);
    expect(validator.validate("ABC")).toBe(true);
    expect(validator.validate("abc")).toBe(false);
    expect(validator.message).toBe(message);
  });
});
