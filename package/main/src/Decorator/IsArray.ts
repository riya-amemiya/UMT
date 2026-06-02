import { addFieldRule } from "@/Decorator/addFieldRule";
import { isArray } from "@/Validate/isArray";

/**
 * Property decorator that requires the decorated field to hold an array.
 * The rule is consumed by {@link validateInstance} and {@link Validatable}.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class Cart {
 *   @IsArray items = [];
 * }
 * ```
 */
export const IsArray = (target: object, propertyKey: string | symbol): void => {
  addFieldRule(target, propertyKey, {
    validate: (value) => isArray(value),
    message: `${String(propertyKey)} must be an array`,
  });
};
