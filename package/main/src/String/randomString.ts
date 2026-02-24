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
  if (length === 0) {
    return "";
  }

  let id = "";
  // 65536 bytes is the limit for getRandomValues in some environments
  // Uint32Array has 4 bytes per element, so 65536 / 4 = 16384 elements.
  const CHUNK_SIZE = 16_384;

  const bufferSize = Math.min(size, CHUNK_SIZE);
  const buffer = new Uint32Array(bufferSize);

  // Calculate the rejection limit to avoid modulo bias.
  // We want to reject values that fall into the remainder range of the random number generator
  // relative to the character set length.
  // 2^32 = 4294967296
  const MAX_UINT32_PLUS_ONE = 4_294_967_296;
  const limit = MAX_UINT32_PLUS_ONE - (MAX_UINT32_PLUS_ONE % length);

  while (id.length < size) {
    globalThis.crypto.getRandomValues(buffer);

    for (let index = 0; index < bufferSize && id.length < size; index++) {
      const value = buffer[index];
      // Rejection sampling: only use values less than the limit to ensure uniform distribution
      if (value < limit) {
        id += char[value % length];
      }
    }
  }
  return id;
};
