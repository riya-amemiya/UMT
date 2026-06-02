import { addFieldRule } from "@/Decorator/addFieldRule";
import { isNumber } from "@/Validate/isNumber";

/**
 * Property decorator that requires the decorated field to hold a number.
 * String representations of numbers are rejected; only genuine numbers pass.
 * The rule is consumed by {@link validateInstance} and {@link Validatable}.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class Product {
 *   @IsNumber price = 0;
 * }
 * ```
 */
export const IsNumber = (
  target: object,
  propertyKey: string | symbol,
): void => {
  addFieldRule(target, propertyKey, {
    validate: (value) => isNumber(value, false),
    message: `${String(propertyKey)} must be a number`,
  });
};
