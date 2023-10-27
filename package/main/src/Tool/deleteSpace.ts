export const deleteSpace = (string_: string) => {
  return string_.replaceAll(/\s/g, "");
};
