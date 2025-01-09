import type { TimeUnit } from "$/time/timeUnit";
import type { TimeUnitShort } from "$/time/timeUnitShort";

const TIME_UNIT_MAP: Record<
  TimeUnit | TimeUnitShort,
  { long: TimeUnit; short: TimeUnitShort }
> = {
  milliseconds: { long: "milliseconds", short: "ms" },
  seconds: { long: "seconds", short: "s" },
  minutes: { long: "minutes", short: "m" },
  hours: { long: "hours", short: "h" },
  ms: { long: "milliseconds", short: "ms" },
  s: { long: "seconds", short: "s" },
  m: { long: "minutes", short: "m" },
  h: { long: "hours", short: "h" },
} as const;

/**
 * Normalize time unit.
 * @param unit Time unit
 * @param to "long" or "short"
 * @returns Normalized time unit
 */
export function normalizeTimeUnit(
  unit: TimeUnit | TimeUnitShort,
  to: "long",
): TimeUnit;
export function normalizeTimeUnit(
  unit: TimeUnit | TimeUnitShort,
  to: "short",
): TimeUnitShort;
export function normalizeTimeUnit(
  unit: TimeUnit | TimeUnitShort,
  to: "long" | "short",
) {
  return TIME_UNIT_MAP[unit][to];
}
