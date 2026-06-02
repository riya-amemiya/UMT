import type { FieldRule } from "@/Decorator/FieldRule";

export interface FieldMeta {
  rules: FieldRule[];
  optional: boolean;
  nullable: boolean;
}
