import { getFieldMeta } from "@/Decorator/getFieldMeta";

/**
 * Marks a field as accepting `null` without running its rules.
 * @param target - The class prototype.
 * @param propertyKey - The field to mark nullable.
 */
export const markNullable = (
  target: object,
  propertyKey: PropertyKey,
): void => {
  getFieldMeta(target, propertyKey).nullable = true;
};
