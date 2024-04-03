import { randomString } from "./randomString";

/**
 * ランダムな文字列を生成する関数を初期化します。
 * @param char ランダムな文字列を生成するための文字列
 * @returns ランダムな文字列を生成する関数 (size: number) => string
 */
export const randomStringInitialization = (
  char = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
) => {
  return (size: number) => randomString(size, char);
};
