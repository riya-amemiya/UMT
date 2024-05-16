let _isBrowser: boolean;
try {
  _isBrowser = typeof window !== "undefined";
} catch {
  _isBrowser = false;
}
/**
 * ブラウザかどうかを判定する
 */
export const isBrowser = _isBrowser;
