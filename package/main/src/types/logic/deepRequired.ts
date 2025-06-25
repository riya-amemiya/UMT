/**
 * Recursively makes all properties in T required, including nested objects.
 * Unlike TypeScript's built-in Required<T> which only affects top-level properties,
 * DeepRequired<T> removes the optional modifier from all levels of nested objects.
 *
 * @template T - The type to make deeply required
 *
 * @example
 * ```typescript
 * interface User {
 *   id?: number;
 *   profile?: {
 *     name?: string;
 *     address?: {
 *       street?: string;
 *       city?: string;
 *     };
 *   };
 * }
 *
 * type RequiredUser = DeepRequired<User>;
 * // Result:
 * // {
 * //   id: number;
 * //   profile: {
 * //     name: string;
 * //     address: {
 * //       street: string;
 * //       city: string;
 * //     };
 * //   };
 * // }
 * ```
 */
// biome-ignore lint/suspicious/noExplicitAny: ignore
export type DeepRequired<T> = T extends Record<PropertyKey, any>
  ? { [P in keyof T]-?: DeepRequired<T[P]> }
  : T;
