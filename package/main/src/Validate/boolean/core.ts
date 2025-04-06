/**
 * Boolean validation core module
 * Provides the base validation functionality for boolean values
 */

import { core } from "@/Validate/core";
import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Creates a boolean validator
 * @param {string} [message] - Custom error message for type validation
 * @returns {Function} - Validator function that checks if the value is a boolean
 */
export const boolean = (message?: string) => {
  return (value: boolean): ValidateCoreReturnType<boolean> =>
    core<boolean>("boolean")(value, [], message);
};
