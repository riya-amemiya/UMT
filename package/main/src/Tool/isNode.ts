/**
 * node環境かどうかを判定します。
 */
export const isNode =
  typeof process !== "undefined" && typeof require !== "undefined";
