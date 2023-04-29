/**
 * @param {number} [timeDifference=0] 時差(時)
 * @returns {Date} 現在時刻
 */
export const now = (timeDifference = 9): Date => {
	const timezoneOffset = new Date().getTimezoneOffset();
	const localTimeDifference = -timezoneOffset / 60;
	const n =
		localTimeDifference === timeDifference
			? timeDifference * 2
			: localTimeDifference + timeDifference;
	return new Date(
		Date.now() +
			(new Date().getTimezoneOffset() +
				(new Date().getTimezoneOffset() === 0 ? timeDifference : n) * 60) *
				60 *
				1000,
	);
};
