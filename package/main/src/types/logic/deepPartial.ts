/**
 * Recursively makes all properties in T optional, including nested objects.
 * Unlike TypeScript's built-in Partial<T> which only affects top-level properties,
 * DeepPartial<T> applies the optional modifier to all levels of nested objects.
 *
 * @template T - The type to make deeply partial
 *
 * @example
 * ```typescript
 * interface User {
 *   id: number;
 *   profile: {
 *     name: string;
 *     address: {
 *       street: string;
 *       city: string;
 *     };
 *   };
 * }
 *
 * type PartialUser = DeepPartial<User>;
 * // Result:
 * // {
 * //   id?: number;
 * //   profile?: {
 * //     name?: string;
 * //     address?: {
 * //       street?: string;
 * //       city?: string;
 * //     };
 * //   };
 * // }
 * ```
 */

// biome-ignore lint/suspicious/noExplicitAny: ignore
export type DeepPartial<T> = T extends Record<PropertyKey, any>
  ? {
      [P in keyof T]?: DeepPartial<T[P]>;
    }
  : T;
