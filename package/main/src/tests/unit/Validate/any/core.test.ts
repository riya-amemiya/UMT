import { any } from "@/Validate/any";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("any validation", () => {
  it("accepts any value", () => {
    const validator = any();
    expect(validator(0).validate).toBe(true);
    expect(validator("").validate).toBe(true);
    expect(validator({}).validate).toBe(true);
    expect(validator(null).validate).toBe(true);
    expect(validator(undefined).validate).toBe(true);
  });

  it("exposes the any tag through the type field", () => {
    const validator = any();
    expect(validator(42).type).toBe("any");
  });

  it("composes inside object()", () => {
    const validator = object({
      data: any(),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      data: { whatever: 123 },
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(any(), string());
    expect(validator(42).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ data: any() }),
      object({ id: number() }),
    );
    expect(validator({ data: 123, id: 1 }).validate).toBe(true);
    expect(validator({ data: { x: 1 }, id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableAny = nullable(any());
    expect(nullableAny(null).validate).toBe(true);
    expect(nullableAny(123).validate).toBe(true);

    const optionalAny = optional(any());
    expect(optionalAny(undefined).validate).toBe(true);
    expect(optionalAny(123).validate).toBe(true);
  });

  it("infers any via SchemaToInterface", () => {
    const validator = any();
    type Schema = SchemaToInterface<typeof validator>;
    const a: Schema = "string";
    const b: Schema = 1;
    const c: Schema = { x: 1 };
    expect(validator(a).validate).toBe(true);
    expect(validator(b).validate).toBe(true);
    expect(validator(c).validate).toBe(true);
  });
});
