import { orderedUnique } from "./setHelpers/orderedUnique";

/**
Removes duplicate values from an array based on a selector function
@param array - The array to process
@param selector - Function that returns the value to compare for uniqueness
@returns A new array with unique values based on the selector
*/
export const uniqBy = <T extends unknown[], K>(
  array: T,
  selector: (item: T[number]) => K,
): T => orderedUnique<T[number], K>([array], selector) as T;
