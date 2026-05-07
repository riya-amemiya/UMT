import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";
import { unknown } from "@/Validate/unknown";

describe("unknown validation", () => {
  it("accepts any value", () => {
    const validator = unknown();
    expect(validator(0).validate).toBe(true);
    expect(validator("").validate).toBe(true);
    expect(validator({}).validate).toBe(true);
    expect(validator(null).validate).toBe(true);
    expect(validator(undefined).validate).toBe(true);
  });

  it("exposes the unknown tag through the type field", () => {
    const validator = unknown();
    expect(validator(42).type).toBe("unknown");
  });

  it("composes inside object()", () => {
    const validator = object({
      data: unknown(),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      data: { whatever: 123 },
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(unknown(), string());
    expect(validator(42).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ data: unknown() }),
      object({ id: number() }),
    );
    expect(validator({ data: 123, id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableUnknown = nullable(unknown());
    expect(nullableUnknown(null).validate).toBe(true);
    expect(nullableUnknown(123).validate).toBe(true);

    const optionalUnknown = optional(unknown());
    expect(optionalUnknown(undefined).validate).toBe(true);
    expect(optionalUnknown(123).validate).toBe(true);
  });

  it("infers unknown via SchemaToInterface", () => {
    const validator = unknown();
    type Schema = SchemaToInterface<typeof validator>;
    const a: Schema = "string";
    const b: Schema = 1;
    expect(validator(a).validate).toBe(true);
    expect(validator(b).validate).toBe(true);
  });
});
