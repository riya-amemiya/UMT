import { toBase64 } from "./toBase64";

/**
Convert string to URL-safe Base64 (RFC 4648 section 5) without padding
@param char String to convert to URL-safe Base64
@returns URL-safe Base64 encoded string using "-" and "_" and no "=" padding
@example encodeBase64Url("Hello World"); // "SGVsbG8gV29ybGQ"
@example encodeBase64Url("???>"); // "Pz8_Pg"
*/
export const encodeBase64Url = (char: string): string =>
  toBase64(char).replaceAll("+", "-").replaceAll("/", "_").replace(/=+$/, "");
