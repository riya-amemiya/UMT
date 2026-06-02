import type { FieldMeta } from "@/Decorator/FieldMeta";

export const storage = new WeakMap<object, Map<PropertyKey, FieldMeta>>();
