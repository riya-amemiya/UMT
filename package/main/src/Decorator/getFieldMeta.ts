import type { FieldMeta } from "@/Decorator/FieldMeta";
import { storage } from "@/Decorator/storage";

/**
 * Returns the metadata entry for a field, creating an empty one on first
 * access.
 * @param target - The class prototype.
 * @param propertyKey - The field to look up.
 * @returns The field's metadata entry.
 */
export const getFieldMeta = (
  target: object,
  propertyKey: PropertyKey,
): FieldMeta => {
  let fields = storage.get(target);
  if (!fields) {
    fields = new Map();
    storage.set(target, fields);
  }
  let meta = fields.get(propertyKey);
  if (!meta) {
    meta = { rules: [], optional: false, nullable: false };
    fields.set(propertyKey, meta);
  }
  return meta;
};
