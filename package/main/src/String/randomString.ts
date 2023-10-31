/**
 * ランダムな文字列を生成します。
 * @param char ランダムな文字列を生成するための文字列
 * @param size ランダムな文字列の長さ
 * @returns ランダムな文字列
 */
export const randomString = (
  char = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
  size = 8,
) => {
  const length = char.length;

  let id = "";
  for (let index = 0; index < size; index++) {
    id += char[Math.trunc(Math.random() * length)];
  }
  return id;
};
