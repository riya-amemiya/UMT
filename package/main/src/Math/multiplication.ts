import { getDecimalLength } from "./getDecimalLength";

/**
 * 誤差のない掛け算
 * @param  {number} x
 * @param  {number} y
 */
export const multiplication = (x: number, y: number) => {
  const n = 10 ** (getDecimalLength(x) + getDecimalLength(y));
  let copyX = x;
  let copyY = y;
  copyX = +`${copyX}`.replace(".", "");
  copyY = +`${copyY}`.replace(".", "");
  return (copyX * copyY) / n;
};
