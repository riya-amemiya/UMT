import { random } from "./random";

/**
 * UUID v7を生成する
 * @example
 * const result = uuidv7()
 */
export const uuidv7 = (): string => {
  const DIGITS = "0123456789abcdef";
  const unixTsMs = Date.now();
  const randA = random(0xf_ff);
  const randBHi = random(0x3f_ff_ff_ff);
  const randBLo = random(0xff_ff_ff_ff);

  const bytes = new Uint8Array(16);
  for (let index = 0; index < 6; index++) {
    bytes[index] = (unixTsMs >>> ((5 - index) * 8)) & 0xff;
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
  for (const [index, byte] of bytes.entries()) {
    uuid += DIGITS[byte >>> 4] + DIGITS[byte & 0xf];
    if (index === 3 || index === 5 || index === 7 || index === 9) {
      uuid += "-";
    }
  }

  return uuid;
};
