import { isPrimeNumber } from "@/Math/isPrimeNumber";

/**
 * 非同期で素数判定を行う
 * @param {number} n - 判定する数
 * @returns {Promise<boolean>} 素数の場合はtrue、そうでない場合はfalse
 * @example isPrimeNumberAsync(2); // Promise<true>
 */
export const isPrimeNumberAsync = async (n: number): Promise<boolean> => {
  return new Promise((resolve) => {
    resolve(isPrimeNumber(n));
  });
};
