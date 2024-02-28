export const fromBase64 = (string_: string): string => {
  return Buffer.from(string_, "base64").toString("utf8");
};
