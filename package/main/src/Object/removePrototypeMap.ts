import { removePrototype } from "@/Object/removePrototype";

/**
 * Creates a new array of objects with prototype polluting properties removed
 * from each item.
 *
 * @param objects - The objects to remove the prototype polluting properties
 * from.
 * @returns A new array with shallowly sanitized objects.
 */
export const removePrototypeMap = <T extends Record<string, unknown>>(
  objects: readonly T[],
): Omit<T, "__proto__" | "constructor" | "prototype">[] => {
  return objects.map((object) => removePrototype(object));
};
