import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Wraps a validator to accept undefined values
 * @template T - The type of value the wrapped validator expects
 * @param {Function} validator - Validator function to make optional
 * @returns {Function} - Validator that passes for undefined or delegates to the wrapped validator
 */
export const optional = <T>(
  validator: (value: T) => ValidateCoreReturnType<T>,
): ((value?: T) => ValidateCoreReturnType<T | undefined>) => {
  const optionalValidator = (
    value?: T,
  ): ValidateCoreReturnType<T | undefined> => {
    if (value === undefined) {
      return {
        validate: true,
        message: "",
        type: "undefined",
      };
    }
    const result = validator(value);
    return result;
  };

  return optionalValidator;
};
