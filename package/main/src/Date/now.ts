/**
 * @param {number} [timeDifference=0] 時差(時)
 * @returns {Date} 現在時刻
 */
export const now = (timeDifference = 9): Date => {
	const n = timeDifference * 2;
	return new Date(
		Date.now() +
			(new Date().getTimezoneOffset() +
				(new Date().getTimezoneOffset() === 0 ? timeDifference : n) * 60) *
				60 *
				1000,
	);
};
