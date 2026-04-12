import { removePrototypeDeep } from "@/Object/removePrototypeDeep";

/**
 * Creates a new array of objects with prototype polluting properties removed
 * recursively from each item.
 *
 * @param objects - The objects to remove the prototype polluting properties
 * from recursively.
 * @returns A new array with deeply sanitized objects.
 */
export const removePrototypeMapDeep = <T extends Record<string, unknown>>(
  objects: readonly T[],
): T[] => {
  return objects.map((object) => removePrototypeDeep(object));
};
