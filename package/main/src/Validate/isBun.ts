let _isBun: boolean;
try {
  // @ts-expect-error: Bun is only defined in Bun runtime
  _isBun = typeof Bun !== "undefined";
} catch {
  _isBun = false;
}
/**
 * bun環境かどうかを判定します。
 */
export const isBun = _isBun;
