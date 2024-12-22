/**
 * bun環境かどうかを判定します。
 */
export const isBun = () => {
  try {
    // @ts-expect-error: Bun is only defined in Bun runtime
    return typeof Bun !== "undefined";
  } catch {
    return false;
  }
};
