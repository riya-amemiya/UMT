import { addFieldRule } from "@/Decorator/addFieldRule";
import { maxValue } from "@/Validate/number/maxValue";

/**
 * Property decorator factory that requires a numeric field to be less than or
 * equal to `maximum`. Non-numeric values fail. Reuses the existing
 * {@link maxValue} validator.
 *
 * @param maximum - The largest accepted value.
 * @returns A property decorator.
 *
 * @example
 * ```typescript
 * class Rating {
 *   @Max(5) stars = 0;
 * }
 * ```
 */
export const Max = (maximum: number) => {
  const check = maxValue(maximum);
  return (target: object, propertyKey: string | symbol): void => {
    addFieldRule(target, propertyKey, {
      validate: (value) => typeof value === "number" && check.validate(value),
      message: `${String(propertyKey)} must be at most ${maximum}`,
    });
  };
};
