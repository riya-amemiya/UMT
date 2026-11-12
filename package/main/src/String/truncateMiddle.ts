/**
Truncate a string in the middle, keeping its head and tail.
The result length (in UTF-16 code units, like truncate) is at most length.
When length is not larger than the separator, the separator is cut to length.
@param string_ - The string to truncate
@param length - The maximum length of the result
@param separator - The text placed between head and tail (default: "...")
@returns The truncated string
@example
truncateMiddle("Hello World", 8); // "Hel...ld"
truncateMiddle("Hello World", 7, "~"); // "Hel~rld"
truncateMiddle("Hello", 10); // "Hello"
truncateMiddle("Hello World", 2); // ".."
*/
export const truncateMiddle = (
  string_: string,
  length: number,
  separator = "...",
): string => {
  if (string_.length <= length) {
    return string_;
  }
  const available = length - separator.length;
  if (available <= 0) {
    return separator.slice(0, Math.max(0, length));
  }
  const headLength = Math.ceil(available / 2);
  const tailLength = available - headLength;
  return (
    string_.slice(0, headLength) +
    separator +
    string_.slice(string_.length - tailLength)
  );
};
