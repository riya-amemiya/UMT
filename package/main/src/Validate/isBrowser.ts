/**
 * ブラウザかどうかを判定する
 */
export const isBrowser = () => {
  try {
    return typeof globalThis !== "undefined";
  } catch {
    return false;
  }
};
