import { division } from "./division";
import { multiplication } from "./multiplication";

/**
 * 素数判定
 * @param  {number} n
 * @returns boolean
 */
export const isPrimeNumber = (n: number) => {
  // Check for trivial cases
  if (n <= 1) {
    return false;
  } else if (n <= 3) {
    return true;
  } else if (n % 2 === 0 || n % 3 === 0) {
    return false;
  }

  // Perform Miller-Rabin primality test
  const s = Math.floor(Math.log2(n - 1));
  const d = (n - 1) / 2 ** s;

  for (let i = 0; i < 10; i++) {
    const a = Math.floor(Math.random() * (n - 2)) + 2;
    // let x = BigInt(a) ** BigInt(d) % BigInt(n);
    let x = division(multiplication(a, d), n, false)[1];

    if (x !== 1 && x !== n - 1) {
      for (let j = 0; j < s - 1; j++) {
        x = division(multiplication(x, x), n, false)[1];

        if (x === 1) {
          return false;
        }

        if (x === n - 1) {
          break;
        }
      }

      if (x !== n - 1) {
        return false;
      }
    }
  }

  return true;
};
