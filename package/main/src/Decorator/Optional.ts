import { markOptional } from "@/Decorator/markOptional";

/**
 * Property decorator that allows the decorated field to be `undefined`. When
 * the value is `undefined`, the other rules on the same field are skipped.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class Profile {
 *   @Optional
 *   @IsString
 *   nickname?: string;
 * }
 * ```
 */
export const Optional = (
  target: object,
  propertyKey: string | symbol,
): void => {
  markOptional(target, propertyKey);
};
