let _isNode: boolean;
try {
  _isNode = typeof process !== "undefined" && typeof require !== "undefined";
} catch {
  _isNode = false;
}
/**
 * node環境かどうかを判定します。
 */
export const isNode = _isNode;
