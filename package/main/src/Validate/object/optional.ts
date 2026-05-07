interface UndefinedReturn {
  validate: boolean;
  message: string;
  type: "undefined";
}

/**
 * Optional validator augmented with a reference to the wrapped validator,
 * used by `required()` to unwrap optional layers when rebuilding a shape
 * @template T - The type of value the wrapped validator expects
 * @template R - The return type of the wrapped validator (preserved so the inner type tag flows through)
 */
export type OptionalValidator<
  T,
  R extends { type: unknown; message: string; validate: boolean } = {
    type: unknown;
    message: string;
    validate: boolean;
  },
> = ((value?: T) => R | UndefinedReturn) & {
  inner: (value: T) => R;
  isOptional: true;
};

/**
 * Wraps a validator to accept undefined values
 * @template T - The type of value the wrapped validator expects
 * @template R - The return type of the wrapped validator (preserved so the inner type tag flows through)
 * @param {Function} validator - Validator function to make optional
 * @returns {OptionalValidator<T, R>} - Validator that passes for undefined or delegates to the wrapped validator
 */
export const optional = <
  T,
  R extends { type: unknown; message: string; validate: boolean },
>(
  validator: (value: T) => R,
): OptionalValidator<T, R> => {
  const optionalValidator = ((value?: T): R | UndefinedReturn => {
    if (value === undefined) {
      return {
        validate: true,
        message: "",
        type: "undefined",
      };
    }
    return validator(value);
  }) as OptionalValidator<T, R>;

  optionalValidator.inner = validator;
  optionalValidator.isOptional = true;

  return optionalValidator;
};
