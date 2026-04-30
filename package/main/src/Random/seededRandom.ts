import { xoshiro256 } from "@/Math/xoshiro256";

const TEXT_ENCODER = new TextEncoder();

const hashStringToSeed = (input: string): number => {
  let hash = 2_166_136_261;
  for (const byte of TEXT_ENCODER.encode(input)) {
    hash ^= byte;
    hash = Math.imul(hash, 16_777_619);
  }
  return hash >>> 0;
};

/**
 * Returns a deterministic PRNG seeded from a number or string. Each call
 * produces a float in `[0, 1)` from the same xoshiro256** stream.
 *
 * @param {number | string} seed - Seed value
 * @returns {() => number} A function that returns sequential pseudo-random floats
 * @example
 * const rand = seededRandom("hello");
 * rand(); // deterministic for the same seed
 */
export const seededRandom = (seed: number | string): (() => number) => {
  const initial =
    typeof seed === "number" ? seed >>> 0 : hashStringToSeed(seed);
  const state: [number, number, number, number] = [
    initial || 1,
    Math.imul(initial, 0x9e_37_79_b9) >>> 0 || 2,
    Math.imul(initial, 0x85_eb_ca_6b) >>> 0 || 3,
    Math.imul(initial, 0xc2_b2_ae_35) >>> 0 || 4,
  ];
  return (): number => xoshiro256(state);
};
