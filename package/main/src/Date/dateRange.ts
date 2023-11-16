import { OneDayMs } from "@/Consts/clock";

export const dateRange = (startDate: Date, endDate: Date): Date[] => {
  const dates = [];
  let currentDate = startDate;
  while (currentDate <= endDate) {
    dates.push(new Date(currentDate));
    currentDate = new Date(currentDate.getTime() + OneDayMs);
  }
  return dates;
};
