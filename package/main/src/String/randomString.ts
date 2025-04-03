/**
 * Generates a random string.
 * @param char String of characters to use for generating random string
 * @param size Length of the random string
 * @returns Random string
 */
export const randomString = (
  size = 8,
  char = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
) => {
  const length = char.length;

  let id = "";
  for (let index = 0; index < size; index++) {
    id += char[Math.trunc(Math.random() * length)];
  }
  return id;
};
