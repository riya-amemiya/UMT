let _isBun: boolean;
try {
  // @ts-ignore
  _isBun = typeof Bun !== "undefined";
} catch {
  _isBun = false;
}
/**
 * bun環境かどうかを判定します。
 */
export const isBun = _isBun;
console.log(isBun);
