import { validateInstance } from "@/Decorator/validateInstance";

// biome-ignore lint/suspicious/noExplicitAny: a class decorator must accept any constructor signature
type AnyConstructor = new (...arguments_: any[]) => object;

/**
 * Class decorator that validates every decorated field when an instance is
 * constructed. Throws an Error joining all failure messages when validation
 * fails, so an invalid instance can never escape its constructor.
 *
 * Validation runs after the wrapped constructor finishes, so it observes the
 * fully initialized instance regardless of the `useDefineForClassFields`
 * setting.
 *
 * @template T - The decorated constructor type.
 * @param base - The class being decorated.
 * @returns A subclass that validates on construction.
 *
 * @example
 * ```typescript
 * @Validatable
 * class Product {
 *   @IsNumber price = 0;
 * }
 * new Product(); // throws when price is not a number
 * ```
 */
export const Validatable = <T extends AnyConstructor>(base: T): T => {
  return class extends base {
    // biome-ignore lint/suspicious/noExplicitAny: forwards arbitrary constructor parameters
    constructor(...arguments_: any[]) {
      super(...arguments_);
      const result = validateInstance(this);
      if (result.type === "error") {
        throw new Error(result.error.map((issue) => issue.message).join("; "));
      }
    }
  };
};
