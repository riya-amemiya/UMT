import { getFieldMeta } from "@/Decorator/getFieldMeta";

/**
 * Marks a field as accepting `undefined` without running its rules.
 * @param target - The class prototype.
 * @param propertyKey - The field to mark optional.
 */
export const markOptional = (
  target: object,
  propertyKey: PropertyKey,
): void => {
  getFieldMeta(target, propertyKey).optional = true;
};
