import { markNullable } from "@/Decorator/markNullable";

/**
 * Property decorator that allows the decorated field to be `null`. When the
 * value is `null`, the other rules on the same field are skipped.
 *
 * @param target - The class prototype.
 * @param propertyKey - The decorated field.
 *
 * @example
 * ```typescript
 * class Profile {
 *   @Nullable
 *   @IsString
 *   bio: string | null = null;
 * }
 * ```
 */
export const Nullable = (
  target: object,
  propertyKey: string | symbol,
): void => {
  markNullable(target, propertyKey);
};
