/**
 * Converts a string to camelCase
 * @param str - The string to convert
 * @returns The camelCase string
 */
export const camelCase = (string_: string): string => {
  return string_
    .replaceAll(/[^a-zA-Z0-9]+(.)/g, (_, char) => char.toUpperCase())
    .replaceAll(/[^a-zA-Z0-9]+$/g, "")
    .replace(/^[A-Z]/, (char) => char.toLowerCase());
};
