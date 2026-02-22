import { addition } from "@/Math/addition";
import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";
import { subtract } from "@/Math/subtract";

/**
 * Generates an array of numbers with the specified length
 * @param length The length of the array
 * @param min The minimum value (default: 0)
 * @param max The maximum value (default: length - 1)
 * @param random Whether to generate random values (default: false)
 * @returns Array of numbers
 * @example generateNumberArray(5); // [0, 1, 2, 3, 4]
 * @example generateNumberArray(5, 10, 14); // [10, 11, 12, 13, 14]
 */
export const generateNumberArray = (
  length: number,
  min = 0,
  max?: number,
  random = false,
): number[] => {
  const actualLength = Math.floor(length);
  const actualMax = max ?? actualLength - 1;
  if (actualLength <= 0) {
    return [];
  }
  if (actualLength === 1) {
    return [min];
  }

  const isIntegerInputs =
    Number.isSafeInteger(min) && Number.isSafeInteger(actualMax);

  if (random) {
    if (isIntegerInputs) {
      const range = actualMax - min + 1;
      return Array.from({ length: actualLength }, () =>
        Math.floor(Math.random() * range) + min,
      );
    }
    return Array.from({ length: actualLength }, () =>
      addition(
        Math.floor(
          multiplication(Math.random(), addition(subtract(actualMax, min), 1)),
        ),
        min,
      ),
    );
  }

  if (isIntegerInputs) {
    const range = actualMax - min;
    const steps = actualLength - 1;
    if (range % steps === 0) {
      const step = range / steps;
      return Array.from({ length: actualLength }, (_, index) =>
        min + index * step,
      );
    }
  }

  const step = division(subtract(actualMax, min), subtract(actualLength, 1));
  return Array.from({ length: actualLength }, (_, index) =>
    addition(min, multiplication(index, step)),
  );
};
