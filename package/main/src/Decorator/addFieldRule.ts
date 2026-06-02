import type { FieldRule } from "@/Decorator/FieldRule";
import { getFieldMeta } from "@/Decorator/getFieldMeta";

/**
 * Registers a validation rule for a class field.
 * @param target - The class prototype receiving the rule.
 * @param propertyKey - The field the rule applies to.
 * @param rule - The rule to register.
 */
export const addFieldRule = (
  target: object,
  propertyKey: PropertyKey,
  rule: FieldRule,
): void => {
  getFieldMeta(target, propertyKey).rules.push(rule);
};
