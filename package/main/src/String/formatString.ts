/**
 * Replaces placeholders in a template string with specified values.
 * Placeholders are in the format {0}, {1}, {2}... and corresponding values are passed as an array.
 *
 * @param template - Template string containing placeholders
 * @param values - Array of values to replace the placeholders in the template
 * @returns String with placeholders replaced with values
 *
 * @example
 * // Returns "Hello, World!"
 * formatString("Hello, {0}!", "World");
 */
export const formatString = (template: string, ...values: unknown[]) => {
  return template.replaceAll(/{(\d+)}/g, (match, index) => {
    return values[index] === undefined ? match : String(values[index]);
  });
};
