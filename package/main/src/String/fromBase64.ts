/**
 * Converts Base64 to string
 * @param {string} base64String - Base64 encoded string
 * @returns Decoded string from Base64
 * @throws {Error} When input is not a valid Base64 string
 */
export const fromBase64 = (base64String: string): string => {
  if (base64String === "") {
    return "";
  }

  try {
    return new TextDecoder().decode(
      Uint8Array.from(
        atob(base64String)
          .split("")
          // biome-ignore lint/style/noNonNullAssertion: it will never be null
          .map((c) => c.codePointAt(0)!),
      ),
    );
  } catch {
    throw new Error("Invalid Base64 string");
  }
};
