import type { ValidateReturnType } from "@/Validate/type";

/**
 * String validation module for UUID strings
 * Provides validation functionality for checking if a string is a valid UUID
 */

/**
 * Creates a validator for checking if a string is a valid UUID
 * @param {number[]} [versions=[4]] - Array of supported UUID versions
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for UUID strings
 */
export const uuid = (
  versions: number[] = [4],
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => {
    return versions.some((version) => {
      // Regular expression for specific UUID version
      const versionRegex = new RegExp(
        `^[\\da-f]{8}-?[\\da-f]{4}-?${version}[\\da-f]{3}-?[89ab][\\da-f]{3}-?[\\da-f]{12}$`,
        "i",
      );
      return versionRegex.test(value);
    });
  },
});
