/**
 * Removes HTML/XML tags from a string, keeping the text content.
 * Unlike `escapeHtml`, which escapes special characters, this deletes the
 * tags entirely. The removal repeats until the string is stable so that
 * nested constructs (e.g. "<sc<script>ript>") cannot survive a single pass.
 * @param {string} string_ - Input string
 * @returns {string} String with tags removed
 * @example
 * stripTags("<p>Hello <b>World</b></p>"); // "Hello World"
 */
export const stripTags = (string_: string): string => {
  let result = string_;
  let previous: string;
  do {
    previous = result;
    result = result.replaceAll(/<[^<>]*>/g, "");
  } while (result !== previous);
  return result;
};
