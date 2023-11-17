import { OneDayMs } from "@/Consts/clock";

/**
 * 指定された開始日と終了日の範囲内の全ての日付を含む配列を生成します。
 *
 * @param {Date} startDate - 範囲の開始日
 * @param {Date} endDate - 範囲の終了日
 * @returns {Date[]} startDateからendDateまでの日付を含むDateオブジェクトの配列
 */
export const dateRange = (startDate: Date, endDate: Date): Date[] => {
  const dates = [];
  let currentDate = startDate;
  while (currentDate <= endDate) {
    dates.push(new Date(currentDate));
    currentDate = new Date(currentDate.getTime() + OneDayMs);
  }
  return dates;
};
