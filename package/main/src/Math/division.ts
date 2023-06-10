import { getDecimalLength } from "./getDecimalLength";
import { valueSwap } from "./valueSwap";

export interface DIVISION {
  (x: number, y: number, isFloor?: true): number;
  (x: number, y: number, isFloor?: false): number[];
}
/**
 * 誤差のない割り算
 * @param  {number} x
 * @param  {number} y
 * @param  {boolean} [isFloor=true]
 * @returns number
 */
export const division = ((x: number, y: number, isFloor = true) => {
<<<<<<< HEAD
  const [decimalLengthX, decimalLengthY] = valueSwap(
    getDecimalLength(x),
    getDecimalLength(y),
  );
  let copyX = x;
  let copyY = y;
  const n =
    decimalLengthX === decimalLengthY
      ? 1
      : 10 ** (decimalLengthY - decimalLengthX);
  copyX = +`${copyX}`.replace(".", "");
  copyY = +`${copyY}`.replace(".", "");
  return isFloor
    ? copyX > copyY
      ? copyX / copyY / n
      : (copyX / copyY) * n
    : [
        copyX > copyY
          ? (copyX - (copyX % copyY)) / copyY / n
          : ((copyX - (copyX % copyY)) / copyY) * n,
        copyX % copyY,
      ];
  // return isFloor ? copyX / copyY : [(copyX - (copyX % copyY)) / copyY, copyX % copyY];
=======
	const [decimalLengthX, decimalLengthY] = valueSwap(
		getDecimalLength(x),
		getDecimalLength(y),
	);
	let copyX = x;
	let copyY = y;
	const n =
		decimalLengthX === decimalLengthY
			? 1
			: 10 ** (decimalLengthY - decimalLengthX);
	copyX = +`${copyX}`.replace(".", "");
	copyY = +`${copyY}`.replace(".", "");
	return isFloor
		? copyX > copyY
			? copyX / copyY / n
			: (copyX / copyY) * n
		: [
				copyX > copyY
					? (copyX - (copyX % copyY)) / copyY / n
					: ((copyX - (copyX % copyY)) / copyY) * n,
				copyX % copyY,
		  ];
	// return isFloor ? copyX / copyY : [(copyX - (copyX % copyY)) / copyY, copyX % copyY];
>>>>>>> 58a3e53 (修正)
}) as DIVISION;
