/**
 * Sanitizes a string by removing non-printable characters.
 * This is useful for cleaning up output from external commands before displaying it in the terminal.
 * It allows printable ASCII characters (space to tilde), newlines, carriage returns, and tabs.
 * @param string_ The string to sanitize.
 * @returns The sanitized string.
 */
export const sanitizeString = (string_: string) => {
  return string_.replaceAll(/[^\t\n\r -~]/g, "");
};
