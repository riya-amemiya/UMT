import { randomString } from "./randomString";

/**
 * Initializes a function that generates random strings.
 * @param char String of characters to use for generating random strings
 * @returns A function that generates random strings (size: number) => string
 */
export const randomStringInitialization = (
  char = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
) => {
  return (size: number) => randomString(size, char);
};
