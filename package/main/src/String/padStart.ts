export const padStart = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  let padding = "";
  while (padding.length + string_.length < targetLength) {
    padding += padString;
  }
  return padding.slice(0, targetLength - string_.length) + string_;
};

export const padEnd = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  let padding = "";
  while (padding.length + string_.length < targetLength) {
    padding += padString;
  }
  return string_ + padding.slice(0, targetLength - string_.length);
};
