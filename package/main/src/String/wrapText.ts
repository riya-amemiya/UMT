const WORD_SEPARATOR = /[ \t\r\f\v]+/;

/**
Wraps text to lines of at most width characters (UTF-16 code units) at word boundaries.
Existing line breaks are kept and each line is wrapped on its own.
Words are separated by spaces, tabs, carriage returns, form feeds and vertical tabs,
and a word longer than width is placed on its own line without being broken.
@param string_ - The text to wrap
@param width - The maximum line width
@returns The wrapped text joined with "\n"
@example
wrapText("The quick brown fox", 10); // "The quick\nbrown fox"
wrapText("a extraordinary b", 5); // "a\nextraordinary\nb"
*/
export const wrapText = (string_: string, width: number): string =>
  string_
    .split("\n")
    .map((paragraph) => {
      const lines: string[] = [];
      for (const word of paragraph.split(WORD_SEPARATOR)) {
        if (word.length === 0) {
          continue;
        }
        const lastIndex = lines.length - 1;
        if (
          lastIndex >= 0 &&
          lines[lastIndex].length + 1 + word.length <= width
        ) {
          lines[lastIndex] += ` ${word}`;
        } else {
          lines.push(word);
        }
      }
      return lines.join("\n");
    })
    .join("\n");
