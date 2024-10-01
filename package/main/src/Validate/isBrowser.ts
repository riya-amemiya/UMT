/**
 * ブラウザかどうかを判定する
 */
export const isBrowser = () => {
  try {
    return typeof window !== "undefined";
  } catch {
    return false;
  }
};
