import type { ValidateCoreReturnType } from "@/Validate/type";

/**
 * Wraps a validator to accept null values
 * @template T - The type of value the wrapped validator expects
 * @param {Function} validator - Validator function to make nullable
 * @returns {Function} - Validator that passes for null or delegates to the wrapped validator
 */
export const nullable = <T>(
  validator: (value: T) => ValidateCoreReturnType<T>,
): ((value: T | null) => ValidateCoreReturnType<T | null>) => {
  const nullableValidator = (
    value: T | null,
  ): ValidateCoreReturnType<T | null> => {
    if (value === null) {
      return {
        validate: true,
        message: "",
        type: "null",
      };
    }
    const result = validator(value);
    return result;
  };

  return nullableValidator;
};
