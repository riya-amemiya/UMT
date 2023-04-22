import { solveEquation } from "../../module/Math/solveEquation";
test("{solveEquation", () => {
	expect(
		solveEquation(
			/*
				x + y = 4
				x + 2y = 10
			*/
			[
				[1, 1],
				[1, 2],
			],
			[4, 10],
		),
	).toEqual(expect.arrayContaining([-2, 6]));
	expect(
		solveEquation(
			/*
				x + 6y = 33
				x + y = 8
			*/
			[
				[1, 6],
				[1, 1],
			],
			[33, 8],
		),
	).toEqual(expect.arrayContaining([3, 5]));
	expect(
		solveEquation(
			/*
				5x-4y+6z=8
				7x-6y+10z=14
				4x+9y+7z=74
			*/
			[
				[5, -4, 6],
				[7, -6, 10],
				[4, 9, 7],
			],
			[8, 14, 74],
		),
	).toEqual(expect.arrayContaining([2, 5, 3]));
});
