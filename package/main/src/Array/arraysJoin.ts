import { orderedUnique } from "./setHelpers/orderedUnique";

/**
Join arrays without duplicates
@param  {unknown[]} array - First array to join
@param  {unknown[]} arrays - Additional arrays to join
@returns {unknown[]} Array with unique elements from all input arrays
@example arraysJoin([1, 2, 3], [2, 3, 4]); // [1, 2, 3, 4]
*/
export const arraysJoin = <A extends unknown[]>(
  array: unknown[],
  ...arrays: unknown[]
) =>
  orderedUnique<unknown>([array, arrays.flat()]).map((value) =>
    value === 0 ? 0 : value,
  ) as A;
