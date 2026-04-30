export interface MaskOptions {
  /** Number of leading characters to keep visible. Default: 1 */
  start?: number;
  /** Number of trailing characters to keep visible. Default: 1 */
  end?: number;
  /** Mask character. Default: "*" */
  char?: string;
}

/**
 * Masks the middle portion of a string with a fill character, preserving the
 * leading and trailing visible counts. Surrogate-pair safe.
 *
 * @param {string} string_ - Input string
 * @param {MaskOptions} [options] - Visible-character counts and mask char
 * @returns {string} Masked string
 * @example
 * mask("1234567890", { start: 2, end: 4 }); // "12****7890"
 * mask("secret"); // "s****t"
 */
export const mask = (string_: string, options: MaskOptions = {}): string => {
  const { start = 1, end = 1, char = "*" } = options;
  const graphemes = [...string_];
  const length = graphemes.length;
  if (start + end >= length) {
    return string_;
  }
  const middleLength = length - start - end;
  return (
    graphemes.slice(0, start).join("") +
    char.repeat(middleLength) +
    graphemes.slice(length - end).join("")
  );
};
