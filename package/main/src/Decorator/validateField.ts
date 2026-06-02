import type { FieldMeta } from "@/Decorator/FieldMeta";

/**
 * Validates a single value against field metadata.
 * @param meta - The field metadata.
 * @param value - The current field value.
 * @returns The failure message, or undefined when the value is valid.
 */
export const validateField = (
  meta: FieldMeta,
  value: unknown,
): string | undefined => {
  if (value === undefined && meta.optional) {
    return undefined;
  }
  if (value === null && meta.nullable) {
    return undefined;
  }
  for (const rule of meta.rules) {
    if (!rule.validate(value)) {
      return rule.message;
    }
  }
  return undefined;
};
