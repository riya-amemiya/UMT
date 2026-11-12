import { OneDayMs, OneHourMs, OneMinuteMs, OneSecondMs } from "@/Consts/clock";

const durationUnits: [label: string, size: number, modulus: number][] = [
  ["d", OneDayMs, Number.POSITIVE_INFINITY],
  ["h", OneHourMs, 24],
  ["m", OneMinuteMs, 60],
  ["s", OneSecondMs, 60],
  ["ms", 1, 1000],
];

/**
Formats a millisecond duration as a compact human-readable string.
Units are days (d), hours (h), minutes (m), seconds (s) and milliseconds (ms),
separated by a space. Zero units are omitted, fractional milliseconds are
truncated, and a zero duration returns "0s". Negative durations keep a
leading "-".

@param {number} milliseconds - Duration in milliseconds
@returns {string} The formatted duration
@example
formatDuration(5_400_000); // "1h 30m"
formatDuration(45_000); // "45s"
formatDuration(183_600_000); // "2d 3h"
formatDuration(1500); // "1s 500ms"
formatDuration(-90_000); // "-1m 30s"
formatDuration(0); // "0s"
*/
export const formatDuration = (milliseconds: number): string => {
  const total = Math.trunc(Math.abs(milliseconds));
  const parts = durationUnits
    .map(
      ([label, size, modulus]) =>
        [label, Math.floor(total / size) % modulus] as const,
    )
    .filter(([, value]) => value > 0)
    .map(([label, value]) => `${value}${label}`);
  if (parts.length === 0) {
    return "0s";
  }
  return `${milliseconds < 0 ? "-" : ""}${parts.join(" ")}`;
};
