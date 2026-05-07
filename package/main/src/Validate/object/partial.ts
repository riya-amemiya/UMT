/**
 * Object validation - partial
 * Returns a new object validator where every key is wrapped in `optional()`,
 * mirroring `Partial<T>` at the type level so consumers see all properties as
 * optional.
 */

import { object, type ObjectShape, type ObjectValidator } from "./core";
import { optional, type OptionalValidator } from "./optional";

type PartialShape<T extends ObjectShape> = {
  [K in keyof T]: OptionalValidator<
    Parameters<T[K]>[0],
    ReturnType<T[K]> extends {
      type: unknown;
      message: string;
      validate: boolean;
    }
      ? ReturnType<T[K]>
      : never
  >;
};

/**
 * Wraps every property validator of `validator` with `optional()`. Validators
 * that are already optional are kept as-is so the wrapper stays idempotent.
 * @template T - Original object shape
 * @param {ObjectValidator<T>} validator - Source object validator
 * @param {string} [message] - Custom error message for the new validator
 * @returns {ObjectValidator<PartialShape<T>>} - New validator where every key accepts undefined
 */
export const partial = <T extends ObjectShape>(
  validator: ObjectValidator<T>,
  message?: string,
): ObjectValidator<PartialShape<T>> => {
  const sourceShape = validator.shape;
  const nextShape = {} as ObjectShape;
  for (const key of Object.keys(sourceShape)) {
    const current = sourceShape[key];
    nextShape[key] =
      (current as { isOptional?: boolean }).isOptional === true
        ? current
        : optional(current);
  }
  return object(nextShape, message) as unknown as ObjectValidator<
    PartialShape<T>
  >;
};
