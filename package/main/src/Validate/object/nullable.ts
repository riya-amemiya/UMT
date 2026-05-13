/**
 * Wraps a validator to accept null values
 * @template T - The type of value the wrapped validator expects
 * @template R - The return type of the wrapped validator (preserved so the inner type tag flows through)
 * @param {Function} validator - Validator function to make nullable
 * @returns {Function} - Validator that passes for null or delegates to the wrapped validator
 */
export const nullable = <
  T,
  R extends { type: unknown; message: string; validate: boolean },
>(
  validator: (value: T) => R,
): ((
  value: T | null,
) => R | { validate: boolean; message: string; type: "null" }) => {
  const nullableValidator = (
    value: T | null,
  ): R | { validate: boolean; message: string; type: "null" } => {
    if (value === null) {
      return {
        validate: true,
        message: "",
        type: "null",
      };
    }
    return validator(value);
  };

  return nullableValidator;
};
