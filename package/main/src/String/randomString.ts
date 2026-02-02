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

  let generated = 0;
  while (generated < size) {
    const needed = size - generated;
    const currentBatchSize = Math.min(needed, CHUNK_SIZE);

    globalThis.crypto.getRandomValues(buffer);

    for (let index = 0; index < currentBatchSize; index++) {
      id += char[buffer[index] % length];
    }
    generated += currentBatchSize;
  }
  return id;
};
