import { addFieldRule } from "@/Decorator/addFieldRule";

/**
 * Property decorator that requires the decorated field to hold a boolean.
 * The rule is consumed by {@link validateInstance} and {@link Validatable}.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class Settings {
 *   @IsBoolean enabled = false;
 * }
 * ```
 */
export const IsBoolean = (
  target: object,
  propertyKey: string | symbol,
): void => {
  addFieldRule(target, propertyKey, {
    validate: (value) => typeof value === "boolean",
    message: `${String(propertyKey)} must be a boolean`,
  });
};
