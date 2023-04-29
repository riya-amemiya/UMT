const { solveEquation } = require("./module/Math/solveEquation");
const { newDateStr, newDateInt } = require("./module/Date/new");
const { now } = require("./module/Date/now");
console.log(
	solveEquation({
		/*
		 * 0.04x - 1.3y = 1.5
		 * -x + y = -6
		 */
		coefficients: [
			[0.04, -1.3],
			[-1, 1],
		],
		constants: [1.5, -6],
	}),
);
const date1 = newDateStr("1995-02-01");
const date2 = newDateInt(1995, 2, 1);
const n = now(9);
console.log("====================================");
console.log(new Date().getUTCHours());
console.log(n);
console.log(date1);
console.log(date1.getHours());
console.log(date1.getUTCDate());
console.log(date1.toString());
console.log(date2);
console.log(date2.getHours());
console.log(date2.getUTCDate());
console.log(date2.toString());
console.log("====================================");
