/**
 * Base64を文字列に変換する
 * @param char Base64
 * @returns 文字列に変換されたBase64
 */
export const fromBase64 = (base64String: string): string => {
  // Base64文字列をバイト配列に変換
  const bytes = Uint8Array.from(
    atob(base64String)
      .split("")
      .map((c) => c.codePointAt(0) as number),
  );
  // UTF-8デコーダーを使用して文字列に変換
  return new TextDecoder().decode(bytes);
};
