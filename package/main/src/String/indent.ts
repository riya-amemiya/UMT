/**
Prefixes every non-empty line with count repetitions of char.
Empty lines are left untouched, which makes this the inverse of dedent.
A negative count is treated as 0.
@param string_ - The string to indent
@param count - The number of times char is repeated per line
@param char - The indentation unit (default: " ")
@returns The indented string
@example
indent("line1\nline2", 2); // "  line1\n  line2"
indent("a\n\nb", 1, "\t"); // "\ta\n\n\tb"
*/
export const indent = (string_: string, count: number, char = " "): string => {
  const prefix = char.repeat(Math.max(0, count));
  return string_
    .split("\n")
    .map((line) => (line.length > 0 ? prefix + line : line))
    .join("\n");
};
