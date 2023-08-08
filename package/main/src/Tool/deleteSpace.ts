export const deleteSpace = (str: string) => {
  return str.replaceAll(/\s/g, "");
};
