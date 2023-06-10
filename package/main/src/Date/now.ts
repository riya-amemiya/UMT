/**
 * タイムゾーンに関係なく、UTCを基準に指定した時差を加えた現在時刻を取得します。
 * @param {number} [timeDifference=9] UTCからの時差を指定します。デフォルトは日本時間です。
 * @returns {Date} 現在時刻
 */
export const now = (timeDifference = 9): Date => {
  return new Date(Date.now() + timeDifference * 60 * 60 * 1000);
};
