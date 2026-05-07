/**
 * File validation core module
 * Provides validation for File / Blob instances. Falls back to `Blob` when
 * the global `File` constructor is unavailable.
 */

import type { ValidateCoreReturnType } from "@/Validate/type";

const hasFileConstructor = typeof File !== "undefined";
const hasBlobConstructor = typeof Blob !== "undefined";

/**
 * Creates a validator that checks whether a value is a `File` (or `Blob` in
 * runtimes where `File` is not available)
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function for File / Blob instances
 */
export const file = (message?: string) => {
  return (value: File): ValidateCoreReturnType<File> => {
    const isFile = hasFileConstructor && value instanceof File;
    const isBlob = hasBlobConstructor && value instanceof Blob;
    if (!(isFile || isBlob)) {
      return {
        validate: false,
        message: message ?? "",
        type: value,
      };
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  };
};
