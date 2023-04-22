import { gcd } from "./gcd";
import { valueSwap } from "./valueSwap";
/**
 * 最小公倍数
 * @param  {number} x
 * @param  {number} y
 * @returns number
 */
export const lcm = (x: number, y: number) => {
	let copyX = x;
	let copyY = y;
	// If either input is 0, the least common multiple is 0
	if (copyX === 0 || copyY === 0) {
		return 0;
	}
	// Swap the values of x and y if x is greater than y
	[copyX, copyY] = valueSwap(copyX, copyY);
	// The least common multiple is x times y divided by their greatest common divisor
	return (copyX / gcd(copyX, copyY)) * copyY;
};
