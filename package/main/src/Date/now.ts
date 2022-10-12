/**
 * @param {number} [timeDifference] 時差(時)
 */
const now = (timeDifference: number = 9) =>
    new Date(
        Date.now() +
            (new Date().getTimezoneOffset() + timeDifference * 60) *
                60 *
                1000,
    );
export { now };
