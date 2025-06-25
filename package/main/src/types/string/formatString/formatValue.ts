/**
 * Valid value types that can be passed to formatString.
 *
 * Includes primitives, objects, arrays, and null/undefined values.
 * Objects and arrays are typically used in indexed mode as placeholder values.
 *
 * @example
 * // Primitive values
 * formatString("{0} {1}", "hello", 42) // string, number
 *
 * @example
 * // Complex values
 * formatString("{0} {1}", { key: "value" }, [1, 2, 3]) // object, array
 */
export type FormatValue =
  | string
  | number
  | boolean
  | Date
  | null
  | undefined
  | Record<string, unknown>
  | unknown[];
