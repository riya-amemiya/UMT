/**
 * node環境かどうかを判定します。
 */
export const isNode = () => {
  try {
    return typeof process !== "undefined" && typeof require !== "undefined";
  } catch {
    return false;
  }
};
