import type { ValidationIssue } from "@/Decorator/ValidationIssue";
import { collectRules } from "@/Decorator/collectRules";
import { validateField } from "@/Decorator/validateField";
import {
  type Result,
  errorFunction,
  successFunction,
} from "@/Error/safeExecute";

/**
 * Validates every decorated field of an instance and aggregates the failures.
 * Reads the rules registered by the field decorators along the prototype
 * chain, so inherited rules are included.
 *
 * @template T - The instance type.
 * @param instance - The object to validate.
 * @returns A success {@link Result} carrying the instance when every field is
 * valid, otherwise an error {@link Result} carrying one issue per failing
 * field.
 *
 * @example
 * ```typescript
 * class User {
 *   @IsString name = "alice";
 *   @IsNumber age = 20;
 * }
 * const result = validateInstance(new User());
 * if (result.type === "error") {
 *   console.log(result.error);
 * }
 * ```
 */
export const validateInstance = <T extends object>(
  instance: T,
): Result<T, ValidationIssue[]> => {
  const fields = collectRules(instance);
  const issues: ValidationIssue[] = [];
  for (const [key, meta] of fields) {
    const message = validateField(
      meta,
      (instance as Record<PropertyKey, unknown>)[key],
    );
    if (message !== undefined) {
      issues.push({ path: String(key), message });
    }
  }
  return issues.length > 0 ? errorFunction(issues) : successFunction(instance);
};
