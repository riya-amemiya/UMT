/**
 * nPr
 * @param  {number} n
 * @param  {number} r
 */
export const nPr = (n: number, r: number) => {
  let copyN = n;
  let copyR = r;
  // nPr;
  if (copyN === 0 || copyR === 0) {
    return NaN;
  }
  let y = copyN;
  let x = 0;
  while (x === 0) {
    if (copyR === 1) {
      y *= copyR;
    }
    copyR--;
    if (copyR === 0) {
      x++;
    } else {
      copyN--;
      if (copyN === 0) {
        x++;
        break;
      }
      y *= copyN;
    }
  }
  if (1 > y) {
    return 0;
  }
  return y;
};
