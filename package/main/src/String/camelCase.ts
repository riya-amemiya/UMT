/**
 * Converts a string to camelCase
 * @param str - The string to convert
 * @returns The camelCase string
 */
export const camelCase = (string_: string): string => {
  return string_
    .replaceAll(/[^\dA-Za-z]+(.)/g, (_, char) => char.toUpperCase())
    .replaceAll(/[^\dA-Za-z]+$/g, "")
    .replace(/^[A-Z]/, (char) => char.toLowerCase());
};
