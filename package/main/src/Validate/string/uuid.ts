import type { ValidateReturnType } from "@/Validate/type";

/**
 * String validation module for UUID strings
 * Provides validation functionality for checking if a string is a valid UUID
 */

// Security: Valid UUID versions are single-digit integers (1-7).
// The version value is interpolated into a RegExp constructor, so untrusted
// input could inject arbitrary regex patterns (regex injection / ReDoS).
// We validate each version is a safe integer in the expected range before use.
const VALID_UUID_VERSIONS = new Set([1, 2, 3, 4, 5, 6, 7]);

/**
 * Creates a validator for checking if a string is a valid UUID
 * @param {number[]} [versions=[4]] - Array of supported UUID versions (1-7)
 * @param {string} [message] - Custom error message for validation failure
 * @returns {ValidateReturnType<string>} - Validator for UUID strings
 */
export const uuid = (
  versions: number[] = [4],
  message?: string,
): ValidateReturnType<string> => {
  // Security: Sanitize version input to prevent regex injection.
  // Only allow known UUID version numbers (integers 1-7) to be interpolated
  // into the RegExp pattern. Reject anything else to block arbitrary regex
  // metacharacter injection via untrusted input.
  const safeVersions = versions.filter((v) => VALID_UUID_VERSIONS.has(v));
  if (safeVersions.length === 0) {
    return {
      type: "string",
      message,
      validate: () => false,
    };
  }

  const versionRegexes = safeVersions.map(
    (version) =>
      new RegExp(
        String.raw`^[\da-f]{8}-?[\da-f]{4}-?${version}[\da-f]{3}-?[89ab][\da-f]{3}-?[\da-f]{12}$`,
        "i",
      ),
  );

  return {
    type: "string",
    message,
    validate: (value) => {
      return versionRegexes.some((regex) => regex.test(value));
    },
  };
};
