import type { FieldMeta } from "@/Decorator/FieldMeta";
import { storage } from "@/Decorator/storage";

/**
 * Collects the field metadata for an instance by walking its prototype
 * chain. Rules declared on a subclass override rules with the same key on a
 * base class.
 * @param instance - The instance to inspect.
 * @returns A map of field keys to their merged metadata.
 */
export const collectRules = (instance: object): Map<PropertyKey, FieldMeta> => {
  const chain: object[] = [];
  let proto: object | null = Object.getPrototypeOf(instance);
  while (proto && proto !== Object.prototype) {
    chain.push(proto);
    proto = Object.getPrototypeOf(proto);
  }
  const merged = new Map<PropertyKey, FieldMeta>();
  for (let index = chain.length - 1; index >= 0; index--) {
    const fields = storage.get(chain[index]);
    if (fields) {
      for (const [key, meta] of fields) {
        merged.set(key, meta);
      }
    }
  }
  return merged;
};
