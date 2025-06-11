import type { FormatData } from "$/string/formatData";
import type { FormatOptions } from "$/string/formatOptions";
import type { FormatValue } from "$/string/formatValue";

/**
 * Detects whether formatString should use indexed or named mode based on arguments.
 *
 * Named mode: First argument is a non-array object
 * Indexed mode: Arguments are treated as array values
 *
 * @param dataOrFirstValue - First argument (object for named mode, value for indexed mode)
 * @param optionsOrSecondValue - Second argument (options for named mode, value for indexed mode)
 * @param restValues - Remaining arguments for indexed mode
 * @returns Object containing data and options for formatting
 *
 * @example
 * // Named mode detection
 * detectMode({ name: "Alice" }) // → { data: { name: "Alice" }, options: {} }
 *
 * @example
 * // Indexed mode detection
 * detectMode("first", "second") // → { data: ["first", "second"], options: {} }
 */
export function detectMode(
  dataOrFirstValue: FormatData | FormatValue | undefined,
  optionsOrSecondValue: FormatOptions | FormatValue | undefined,
  restValues: FormatValue[],
): {
  data: unknown;
  options: FormatOptions;
} {
  const isFirstArgumentObject =
    dataOrFirstValue !== undefined &&
    typeof dataOrFirstValue === "object" &&
    dataOrFirstValue !== null &&
    !Array.isArray(dataOrFirstValue) &&
    !(dataOrFirstValue instanceof Date);

  const isSecondArgumentOptions =
    optionsOrSecondValue !== undefined &&
    typeof optionsOrSecondValue === "object" &&
    optionsOrSecondValue !== null &&
    !Array.isArray(optionsOrSecondValue) &&
    !(optionsOrSecondValue instanceof Date) &&
    "formatters" in optionsOrSecondValue;

  if (
    isFirstArgumentObject &&
    optionsOrSecondValue === undefined &&
    restValues.length === 0
  ) {
    return {
      data: dataOrFirstValue,
      options: {},
    };
  }

  if (
    isFirstArgumentObject &&
    isSecondArgumentOptions &&
    restValues.length === 0
  ) {
    return {
      data: dataOrFirstValue,
      options: optionsOrSecondValue,
    };
  }

  const allValues: unknown[] = [];
  if (dataOrFirstValue !== undefined) {
    allValues.push(dataOrFirstValue);
  }
  if (optionsOrSecondValue !== undefined) {
    allValues.push(optionsOrSecondValue);
  }
  allValues.push(...restValues);

  return {
    data: allValues,
    options: {},
  };
}
