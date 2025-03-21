import { isValueNaN } from "@/Validate/isValueNaN";
/**
 * Flexible function to convert various inputs to numbers whenever possible
 *
 * @param value - Value to convert (any type)
 * @returns Converted number, or NaN if conversion is not possible
 *
 * @description
 * This function has the following features:
 * 1. Converts null, undefined, and empty string to 0
 * 2. Returns numbers as-is if already a number type
 * 3. Properly handles infinity (Infinity, -Infinity)
 * 4. Supports string representations of hexadecimal (0x), octal (0o), and binary (0b)
 * 5. Properly parses floating-point number strings
 * 6. Extracts numbers from strings that start with numbers when possible
 * 7. Returns NaN if none of the above conditions are met
 *
 * @example
 * flexibleNumberConversion(123)        // 123
 * flexibleNumberConversion("456")      // 456
 * flexibleNumberConversion("78.9")     // 78.9
 * flexibleNumberConversion("3.14e2")   // 314
 * flexibleNumberConversion("0xFF")     // 255
 * flexibleNumberConversion("42px")     // 42
 * flexibleNumberConversion("")         // 0
 * flexibleNumberConversion("not a number") // NaN
 */
export const flexibleNumberConversion = (value: unknown): number => {
  // Return NaN for objects
  if (typeof value === "object" && value !== null) {
    return Number.NaN;
  }

  // Handle special cases
  if (value === null || value === undefined || value === "") {
    return 0;
  }

  // Handle values already of type number
  if (typeof value === "number" && !isValueNaN(value)) {
    return value;
  }

  // Convert to string and process
  const stringValue = String(value).trim().toLowerCase();

  // Handle infinity
  if (stringValue === "infinity" || stringValue === "+infinity") {
    return Number.POSITIVE_INFINITY;
  }
  if (stringValue === "-infinity") {
    return Number.NEGATIVE_INFINITY;
  }

  // Handle special base notations (hex, octal, binary)
  if (
    stringValue.startsWith("0x") ||
    stringValue.startsWith("0o") ||
    stringValue.startsWith("0b")
  ) {
    return Number(stringValue);
  }

  // Parse as floating point number
  const floatValue = Number.parseFloat(stringValue);
  if (!isValueNaN(floatValue)) {
    return floatValue;
  }

  // When conversion is not possible
  return Number.NaN;
};
