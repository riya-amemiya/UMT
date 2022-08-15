/**
 * 現在時刻を取得(時差考慮)
 * @param {number} [timeDifferencet=0] 時差(時)
 */
const now = (timeDifference = 9) =>
    new Date(
        Date.now() +
            (new Date().getTimezoneOffset() + timeDifference * 60) *
                60 *
                1000,
    );
export default now;
