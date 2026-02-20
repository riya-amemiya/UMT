import { unwrap } from "@/Tool/unwrap";

/**
 * Converts Base64 to string
 * @param {string} base64String - Base64 encoded string
 * @returns Decoded string from Base64
 */
export const fromBase64 = (base64String: string): string => {
  if (base64String === "") {
    return "";
  }

  return new TextDecoder().decode(
    Uint8Array.from(
      atob(base64String)
        .split("")
        .map((c) => unwrap(c.codePointAt(0), "panic: invalid base64 string")),
    ),
  );
};
