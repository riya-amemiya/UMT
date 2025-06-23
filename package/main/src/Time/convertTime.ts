import type { TimeUnit } from "$/time/timeUnit";
import type { TimeUnitShort } from "$/time/timeUnitShort";
import { division, multiplication } from "@/Math";
import { normalizeTimeUnit } from "@/Time/normalizeTimeUnit";

/**
 * Defines conversion rates between time units
 */
const conversionRates: Record<TimeUnit, number> = {
  milliseconds: 1,
  seconds: 1000,
  minutes: 60_000,
  hours: 3_600_000,
};

/**
 * Converts time between different units
 * @param value Value to convert (string or number)
 * @param fromUnit Source time unit
 * @param toUnit Target time unit
 * @returns Converted value (number)
 * @throws {Error} If the input value is invalid
 */
export const convertTime = (
  value: string | number,
  fromUnit: TimeUnit | TimeUnitShort,
  toUnit: TimeUnit | TimeUnitShort,
): number => {
  const numericValue =
    typeof value === "string" ? Number.parseFloat(value) : value;
  const normalizedFromUnit = normalizeTimeUnit(fromUnit, "long");
  const normalizedToUnit = normalizeTimeUnit(toUnit, "long");
  const milliseconds = multiplication(
    numericValue,
    conversionRates[normalizedFromUnit],
  );
  return division(milliseconds, conversionRates[normalizedToUnit]);
};
