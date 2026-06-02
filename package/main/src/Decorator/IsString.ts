import { addFieldRule } from "@/Decorator/addFieldRule";
import { isString } from "@/Validate/isString";

/**
 * Property decorator that requires the decorated field to hold a string.
 * The rule is consumed by {@link validateInstance} and {@link Validatable}.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class User {
 *   @IsString name = "";
 * }
 * ```
 */
export const IsString = (
  target: object,
  propertyKey: string | symbol,
): void => {
  addFieldRule(target, propertyKey, {
    validate: (value) => isString(value),
    message: `${String(propertyKey)} must be a string`,
  });
};
