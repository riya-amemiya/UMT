/**
 * Convert string to Base64
 * @param char String to convert to Base64
 * @returns Base64 encoded string
 */
export const toBase64 = (char: string): string =>
  btoa(
    encodeURIComponent(char).replaceAll(/%([\dA-F]{2})/g, (_, p1) =>
      String.fromCodePoint(Number.parseInt(p1, 16)),
    ),
  );
