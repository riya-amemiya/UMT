/**
 * Removes the minimum common leading whitespace from each line of the input.
 * Works as a plain function or as a tagged template literal.
 *
 * @param {string | TemplateStringsArray} string_ - Input string or template strings array
 * @param {...unknown[]} values - Interpolated values when used as a tag
 * @returns {string} Dedented string
 * @example
 * dedent(`
 *   line1
 *     line2
 * `); // "\nline1\n  line2\n"
 *
 * @example
 * dedent`
 *   value: ${1}
 * `; // "\nvalue: 1\n"
 */
export const dedent = (
  string_: string | TemplateStringsArray,
  ...values: unknown[]
): string => {
  let raw: string;
  if (typeof string_ === "string") {
    raw = string_;
  } else {
    let assembled = "";
    let index = 0;
    for (const part of string_) {
      assembled += part;
      if (index < values.length) {
        assembled += String(values[index]);
      }
      index += 1;
    }
    raw = assembled;
  }

  const lines = raw.split("\n");
  let minIndent = Number.POSITIVE_INFINITY;
  for (const line of lines) {
    if (line.trim().length === 0) {
      continue;
    }
    let indent = 0;
    while (
      indent < line.length &&
      (line[indent] === " " || line[indent] === "\t")
    ) {
      indent += 1;
    }
    if (indent < minIndent) {
      minIndent = indent;
    }
  }
  if (!Number.isFinite(minIndent)) {
    return raw;
  }
  return lines.map((line) => line.slice(minIndent)).join("\n");
};
