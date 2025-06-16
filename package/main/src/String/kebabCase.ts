/**
 * Converts a string to kebab-case
 * @param str - The string to convert
 * @returns The kebab-case string
 */
export const kebabCase = (string_: string): string => {
  return (
    string_
      // Insert dash between lowercase and uppercase
      .replaceAll(/([a-z])([A-Z])/g, "$1-$2")
      // Insert dash between sequences of uppercase letters and following lowercase
      .replaceAll(/([A-Z])([A-Z][a-z])/g, "$1-$2")
      // Replace spaces and underscores with dashes
      .replaceAll(/[\s_]+/g, "-")
      // Remove special characters except alphanumeric and dashes
      .replaceAll(/[^a-zA-Z0-9-]/g, "-")
      // Remove multiple consecutive dashes
      .replaceAll(/-+/g, "-")
      // Remove leading and trailing dashes
      .replaceAll(/^-|-$/g, "")
      .toLowerCase()
  );
};
