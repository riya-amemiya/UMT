/**
 * ブラウザかどうかを判定する
 */
let _isBrowser: boolean;
try {
  _isBrowser = typeof window !== "undefined";
} catch {
  _isBrowser = false;
}
export const isBrowser = _isBrowser;
