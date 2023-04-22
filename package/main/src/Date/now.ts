/**
 * @param {number} [timeDifference=9] 時差(時)
 * @returns {Date} 現在時刻
 */
export const now = (timeDifference = 9): Date =>
    new Date(
        Date.now() +
            (new Date().getTimezoneOffset() + timeDifference * 60) *
                60 *
                1000,
    );
