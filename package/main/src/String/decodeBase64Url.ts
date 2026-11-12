import { fromBase64 } from "./fromBase64";

/**
Converts URL-safe Base64 (RFC 4648 section 5) to string
@param {string} base64UrlString - URL-safe Base64 encoded string, with or without "=" padding
@returns Decoded string from URL-safe Base64
@example decodeBase64Url("SGVsbG8gV29ybGQ"); // "Hello World"
@example decodeBase64Url("SGVsbG8gV29ybGQ="); // "Hello World"
*/
export const decodeBase64Url = (base64UrlString: string): string =>
  fromBase64(
    base64UrlString.replaceAll("-", "+").replaceAll("_", "/") +
      "=".repeat((4 - (base64UrlString.length % 4)) % 4),
  );
