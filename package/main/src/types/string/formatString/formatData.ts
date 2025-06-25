import type { FormatValue } from "./formatValue";

/**
 * Data structure for formatString input.
 *
 * Can be either an object for named placeholders or an array for indexed placeholders.
 *
 * @example
 * // Object for named placeholders
 * const namedData: FormatData = { name: "Alice", age: 25 };
 * formatString("Hello {name}, age {age}", namedData);
 *
 * @example
 * // Array for indexed placeholders
 * const indexedData: FormatData = ["Alice", 25];
 * formatString("Hello {0}, age {1}", ...indexedData);
 */
export type FormatData = Record<string, unknown> | FormatValue[];
