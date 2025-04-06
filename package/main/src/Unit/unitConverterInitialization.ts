import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";

/**
 * Unit converter initialization function
 * Creates a converter function that can convert between different units using base unit ratios
 * @template T Object type containing unit to base unit ratios
 * @template K Key type of the ratios object (string, number, or symbol)
 * @param toBaseUnitRatios Object containing conversion ratios to the base unit
 * @returns A function that converts values between units
 * @example
 * const lengthConverter = unitConverterInitialization({
 *   meters: 1,      // base unit
 *   kilometers: 1000,
 *   centimeters: 0.01
 * });
 * lengthConverter(5, "kilometers", "meters"); // returns 5000
 */
export const unitConverterInitialization =
  <
    T extends {
      [k in K]: number;
    },
    K extends string | number | symbol,
  >(
    toBaseUnitRatios: T,
  ) =>
  (value: number, from: keyof T, to: keyof T) =>
    multiplication(
      division(value, toBaseUnitRatios[from]),
      toBaseUnitRatios[to],
    );
