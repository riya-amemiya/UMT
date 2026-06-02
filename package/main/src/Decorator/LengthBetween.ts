import { addFieldRule } from "@/Decorator/addFieldRule";

/**
 * Property decorator factory that requires a string or array field to have a
 * length within the inclusive range `[minimum, maximum]`. Values without a
 * numeric length fail.
 *
 * @param minimum - The smallest accepted length.
 * @param maximum - The largest accepted length.
 * @returns A property decorator.
 *
 * @example
 * ```typescript
 * class User {
 *   @LengthBetween(1, 20) name = "";
 * }
 * ```
 */
export const LengthBetween = (minimum: number, maximum: number) => {
  return (target: object, propertyKey: string | symbol): void => {
    addFieldRule(target, propertyKey, {
      validate: (value) =>
        (typeof value === "string" || Array.isArray(value)) &&
        value.length >= minimum &&
        value.length <= maximum,
      message: `${String(propertyKey)} length must be between ${minimum} and ${maximum}`,
    });
  };
};
