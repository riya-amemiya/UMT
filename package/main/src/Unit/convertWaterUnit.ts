import type { WaterUnit } from "$/unit/waterUnit";
import { division, multiplication } from "@/Math";

const toBaseUnitRatios: Record<WaterUnit, number> = {
  L: 1,
  mL: 1000,
  cup: 4.227,
  m3: 0.001,
};

/**
 * Convert between different units of water volume.
 * @param value The value to convert
 * @param from Source unit
 * @param to Target unit
 * @returns Converted value
 */
export const convertWaterUnit = (
  value: number,
  from: WaterUnit,
  to: WaterUnit,
) =>
  multiplication(division(value, toBaseUnitRatios[from]), toBaseUnitRatios[to]);
