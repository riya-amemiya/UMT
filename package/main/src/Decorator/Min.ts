import { addFieldRule } from "@/Decorator/addFieldRule";
import { minValue } from "@/Validate/number/minValue";

/**
 * Property decorator factory that requires a numeric field to be greater than
 * or equal to `minimum`. Non-numeric values fail. Reuses the existing
 * {@link minValue} validator.
 *
 * @param minimum - The smallest accepted value.
 * @returns A property decorator.
 *
 * @example
 * ```typescript
 * class Account {
 *   @Min(0) balance = 0;
 * }
 * ```
 */
export const Min = (minimum: number) => {
  const check = minValue(minimum);
  return (target: object, propertyKey: string | symbol): void => {
    addFieldRule(target, propertyKey, {
      validate: (value) => typeof value === "number" && check.validate(value),
      message: `${String(propertyKey)} must be at least ${minimum}`,
    });
  };
};
