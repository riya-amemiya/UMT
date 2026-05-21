// biome-ignore-all lint/suspicious/noControlCharactersInRegex: ANSI escape sequences are control characters by definition

/**
 * Removes ANSI escape sequences (e.g. terminal color codes) from a string,
 * keeping the visible text. Unlike `sanitizeString`, which strips all
 * non-printable characters, this targets only ANSI sequences.
 * @param {string} string_ - Input string
 * @returns {string} String without ANSI escape sequences
 * @example
 * stripAnsi("\u001B[31mred\u001B[0m"); // "red"
 */
export const stripAnsi = (string_: string): string =>
  string_.replaceAll(
    // eslint-disable-next-line no-control-regex
    /[\u001B\u009B][#();?[\]]*(?:(?:(?:(?:;[\w#%&./:=?@~-]+)*|[\dA-Za-z]+(?:;[\w#%&./:=?@~-]*)*)?\u0007)|(?:(?:\d{1,4}(?:;\d{0,4})*)?[\d<=>A-PR-TZcf-nq-uy~]))/g,
    "",
  );
