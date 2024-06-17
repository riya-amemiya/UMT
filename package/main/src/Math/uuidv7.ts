import { random } from "./random";

/**
 * UUID v7を生成する
 * @example
 * const result = uuidv7()
 */
export const uuidv7 = (): string => {
  const DIGITS = "0123456789abcdef";
  const unixTsMs = Date.now();
  const randA = random(0xfff);
  const randBHi = random(0x3fffffff);
  const randBLo = random(0xffffffff);

  const bytes = new Uint8Array(16);
  for (let i = 0; i < 6; i++) {
    bytes[i] = (unixTsMs >>> ((5 - i) * 8)) & 0xff;
  }
  bytes[6] = 0x70 | (randA >>> 8);
  bytes[7] = randA & 0xff;
  bytes[8] = 0x80 | (randBHi >>> 24);
  bytes[9] = (randBHi >>> 16) & 0xff;
  bytes[10] = (randBHi >>> 8) & 0xff;
  bytes[11] = randBHi & 0xff;
  bytes[12] = (randBLo >>> 24) & 0xff;
  bytes[13] = (randBLo >>> 16) & 0xff;
  bytes[14] = (randBLo >>> 8) & 0xff;
  bytes[15] = randBLo & 0xff;

  let uuid = "";
  for (let i = 0; i < bytes.length; i++) {
    uuid += DIGITS[bytes[i] >>> 4] + DIGITS[bytes[i] & 0xf];
    if (i === 3 || i === 5 || i === 7 || i === 9) {
      uuid += "-";
    }
  }

  return uuid;
};
