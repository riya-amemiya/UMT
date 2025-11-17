import { toBase64, fromBase64 } from "@/String";
import { formatString } from "@/String/formatString";
import { trimCharacters } from "@/String/trimCharacters";
import { camelCase, kebabCase } from "@/String";
import { string } from "@/Validate/string";
import {
  validateEmail,
  regexMatch,
  minLength,
  maxLength,
  uuid,
} from "@/Validate/string";

/**
 * Integration tests for String transformation and validation functions
 *
 * Tests the interaction between string manipulation and validation:
 * - Base64 encoding/decoding with validation
 * - Format transformations with validation
 * - String cleaning and validation workflows
 */
describe("Integration test for string transformation and validation", () => {
  it("should encode/decode Base64 and validate the results", () => {
    const originalText = "Hello, World! 日本語";
    const encoded = toBase64(originalText);
    const decoded = fromBase64(encoded);

    const base64Validator = string([regexMatch(/^[A-Za-z0-9+/=]+$/)]);

    expect(base64Validator(encoded).validate).toBe(true);
    expect(decoded).toBe(originalText);
  });

  it("should format strings and validate email addresses", () => {
    const emailTemplate = "{0}@{1}.{2}";
    const generatedEmail = formatString(
      emailTemplate,
      "user",
      "example",
      "com",
    );

    const emailValidator = string([validateEmail()]);
    const result = emailValidator(generatedEmail);

    expect(result.validate).toBe(true);
    expect(generatedEmail).toBe("user@example.com");
  });

  it("should clean strings and validate length constraints", () => {
    const messyString = "!!!Hello World!!!";
    const cleaned = trimCharacters(messyString, "!");

    const lengthValidator = string([minLength(5), maxLength(15)]);
    const result = lengthValidator(cleaned);

    expect(result.validate).toBe(true);
    expect(cleaned).toBe("Hello World");
  });

  it("should transform case and validate format", () => {
    const originalString = "user-profile-settings";
    const camelCased = camelCase(originalString);
    const kebabCased = kebabCase(camelCased);

    const camelCaseValidator = string([regexMatch(/^[a-z][a-zA-Z0-9]*$/)]);
    const kebabCaseValidator = string([regexMatch(/^[a-z]+(-[a-z]+)*$/)]);

    expect(camelCaseValidator(camelCased).validate).toBe(true);
    expect(kebabCaseValidator(kebabCased).validate).toBe(true);
    expect(kebabCased).toBe(originalString);
  });

  it("should validate UUID after string manipulation", () => {
    const uuidTemplate = "{0}-{1}-{2}-{3}-{4}";
    const generatedUuid = formatString(
      uuidTemplate,
      "550e8400",
      "e29b",
      "41d4",
      "a716",
      "446655440000",
    );

    const uuidValidator = string([uuid()]);
    const result = uuidValidator(generatedUuid);

    expect(result.validate).toBe(true);
  });

  it("should handle complex string workflows with multiple validations", () => {
    const userData = {
      raw: "  john.doe@EXAMPLE.COM  ",
      template: "User: {0}, Domain: {1}",
    };

    const trimmed = userData.raw.trim();
    const lowercased = trimmed.toLowerCase();

    const emailValidator = string([validateEmail()]);
    const emailResult = emailValidator(lowercased);

    if (emailResult.validate) {
      const [user, domain] = lowercased.split("@");
      const formatted = formatString(userData.template, user, domain);

      const formattedValidator = string([minLength(10)]);
      const formattedResult = formattedValidator(formatted);

      expect(formattedResult.validate).toBe(true);
      expect(formatted).toBe("User: john.doe, Domain: example.com");
    }
  });

  it("should validate Base64 encoded JSON strings", () => {
    const jsonData = { user: "test", id: 123 };
    const jsonString = JSON.stringify(jsonData);
    const encoded = toBase64(jsonString);

    const base64Validator = string([regexMatch(/^[A-Za-z0-9+/=]+$/)]);
    expect(base64Validator(encoded).validate).toBe(true);

    const decoded = fromBase64(encoded);
    const parsedData = JSON.parse(decoded);

    expect(parsedData).toEqual(jsonData);
  });

  it("should transform and validate configuration keys", () => {
    const configKeys = [
      "database-connection-string",
      "api-key-value",
      "max-retry-attempts",
    ];

    const transformedKeys = configKeys.map((key) => ({
      original: key,
      camel: camelCase(key),
      validated: string([regexMatch(/^[a-z][a-zA-Z0-9]*$/)])(camelCase(key)),
    }));

    for (const { original, camel, validated } of transformedKeys) {
      expect(validated.validate).toBe(true);
      expect(kebabCase(camel)).toBe(original);
    }
  });
});
