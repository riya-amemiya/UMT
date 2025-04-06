import { padStart } from "@/String/padStart";

/**
 * Get timezone offset string in format "+HH:mm" or "-HH:mm"
 * @param {Date} instance - The date object to get timezone offset from
 * @return {string} - The timezone offset string (e.g. "+09:00" for JST)
 */
export const getTimezoneOffsetString = (instance: Date): string => {
  const negMinutes = -instance.getTimezoneOffset();
  const minutes = Math.abs(negMinutes);
  const hourOffset = Math.floor(minutes / 60);
  const minuteOffset = minutes % 60;
  return `${negMinutes >= 0 ? "+" : "-"}${padStart(
    String(hourOffset),
    2,
    "0",
  )}:${padStart(String(minuteOffset), 2, "0")}`;
};
