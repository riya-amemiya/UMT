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
  if (actualLength <= 0) {
    return [];
  }
  if (actualLength === 1) {
    return [min];
  }

  const actualMax = max ?? actualLength - 1;
  const result = new Array<number>(actualLength);

  const isIntegerInputs =
    Number.isSafeInteger(min) && Number.isSafeInteger(actualMax);

  if (random) {
    if (isIntegerInputs) {
      const range = actualMax - min + 1;
      for (let index = 0; index < actualLength; index++) {
        result[index] = Math.floor(Math.random() * range) + min;
      }
      return result;
    }
    for (let index = 0; index < actualLength; index++) {
      result[index] = addition(
        Math.floor(
          multiplication(Math.random(), addition(subtract(actualMax, min), 1)),
        ),
        min,
      );
    }
    return result;
  }

  if (isIntegerInputs) {
    const range = actualMax - min;
    const steps = actualLength - 1;
    if (range % steps === 0) {
      const step = range / steps;
      for (let index = 0; index < actualLength; index++) {
        result[index] = min + index * step;
      }
      return result;
    }
  }

  const step = division(subtract(actualMax, min), subtract(actualLength, 1));
  for (let index = 0; index < actualLength; index++) {
    result[index] = addition(min, multiplication(index, step));
  }
  return result;
};
