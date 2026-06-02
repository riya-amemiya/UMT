import { addFieldRule } from "@/Decorator/addFieldRule";
import type { StandardSchemaV1 } from "@/Validate/standardSchema";

/**
 * Property decorator factory that validates a field against any Standard
 * Schema V1 validator, including every UMT validator (`number()`,
 * `arrayOf()`, `object()`, ...). Only synchronous validators are supported;
 * a validator that returns a Promise is treated as a failure.
 *
 * @template Output - The validated output type.
 * @param validator - A Standard Schema V1 compatible validator.
 * @returns A property decorator.
 *
 * @example
 * ```typescript
 * class Box {
 *   @Schema(arrayOf(number())) values = [];
 * }
 * ```
 */
export const Schema = <Output>(
  validator: StandardSchemaV1<unknown, Output>,
) => {
  return (target: object, propertyKey: string | symbol): void => {
    addFieldRule(target, propertyKey, {
      validate: (value) => {
        const result = validator["~standard"].validate(value);
        if (result instanceof Promise) {
          return false;
        }
        return result.issues === undefined;
      },
      message: `${String(propertyKey)} failed schema validation`,
    });
  };
};
