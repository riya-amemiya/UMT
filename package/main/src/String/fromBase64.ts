import { unwrap } from "@/Tool/unwrap";

const TEXT_DECODER = new TextDecoder();

/**
Converts Base64 to string
@param {string} base64String - Base64 encoded string
@returns Decoded string from Base64
*/
export const fromBase64 = (base64String: string): string => {
  if (base64String === "") {
    return "";
  }

  const binary = atob(base64String);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index++) {
    bytes[index] = unwrap(
      binary.codePointAt(index),
      "panic: invalid base64 string",
    );
  }
  return TEXT_DECODER.decode(bytes);
};
