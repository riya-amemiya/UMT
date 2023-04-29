/**
 * @param {number} [timeDifference=0] 時差(時)
 * @returns {Date} 現在時刻
 */
export const now = (timeDifference = 9): Date => {
	return new Date(Date.now() + timeDifference * 60 * 60 * 1000);
};
