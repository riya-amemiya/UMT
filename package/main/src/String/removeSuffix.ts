/**
 * Removes the given suffix from the end of a string once, if present.
 * Unlike `trimEndCharacters`, which removes any of a set of characters
 * repeatedly, this removes the exact suffix a single time.
 * @param {string} string_ - Input string
 * @param {string} suffix - Suffix to remove
 * @returns {string} String without the trailing suffix
 * @example
 * removeSuffix("file.txt", ".txt"); // "file"
 * removeSuffix("file", ".txt"); // "file"
 */
export const removeSuffix = (string_: string, suffix: string): string =>
  suffix !== "" && string_.endsWith(suffix)
    ? string_.slice(0, -suffix.length)
    : string_;
