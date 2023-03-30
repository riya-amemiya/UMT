const { solveEquation } = require("./module/Math/solveEquation");
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
