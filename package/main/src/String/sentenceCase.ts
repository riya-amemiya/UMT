import { capitalize } from "./capitalize";
import { words } from "./words";

/**
Converts a string to Sentence case: the first word capitalized,
every other word lower case, all separated by single spaces.
Words are split with words.
@param {string} string_ - Input string
@returns {string} Sentence case string
@example
sentenceCase("helloWorld foo-bar"); // "Hello world foo bar"
sentenceCase("XMLHttpRequest"); // "Xml http request"
sentenceCase(""); // ""
*/
export const sentenceCase = (string_: string): string =>
  words(string_)
    .map((word, index) =>
      index === 0 ? capitalize(word.toLowerCase()) : word.toLowerCase(),
    )
    .join(" ");
