import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { partial } from "@/Validate/object/partial";
import { required } from "@/Validate/object/required";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("required", () => {
  const baseValidator = object({
    name: optional(string()),
    age: optional(number()),
    active: optional(boolean()),
  });

  it("requires all keys when applied to a fully optional shape", () => {
    const validator = required(baseValidator);
    const valid: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
      active: true,
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("rejects when any required property is missing", () => {
    const validator = required(baseValidator);
    // @ts-expect-error missing age and active
    expect(validator({ name: "John" }).validate).toBe(false);
  });

  it("undoes a partial() wrapper", () => {
    const original = object({
      name: string(),
      age: number(),
    });
    const validator = required(partial(original));
    const valid: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
    };
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error missing age
    expect(validator({ name: "John" }).validate).toBe(false);
  });

  it("keeps non-optional properties unchanged", () => {
    const mixed = object({
      name: string(),
      age: optional(number()),
    });
    const validator = required(mixed);
    const valid: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
    };
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error missing age
    expect(validator({ name: "John" }).validate).toBe(false);
  });

  it("composes with union", () => {
    const validator = union(required(baseValidator), string());
    expect(validator("hello").validate).toBe(true);
    expect(validator({ name: "John", age: 30, active: true }).validate).toBe(
      true,
    );
  });

  it("composes with intersection", () => {
    const validator = intersection(
      required(baseValidator),
      object({ id: number() }),
    );
    expect(
      validator({ name: "John", age: 30, active: true, id: 1 }).validate,
    ).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableReq = nullable(required(baseValidator));
    expect(nullableReq(null).validate).toBe(true);
    expect(nullableReq({ name: "John", age: 30, active: true }).validate).toBe(
      true,
    );

    const optionalReq = optional(required(baseValidator));
    expect(optionalReq(undefined).validate).toBe(true);
    expect(optionalReq({ name: "John", age: 30, active: true }).validate).toBe(
      true,
    );
  });

  it("infers Required<T> via SchemaToInterface", () => {
    const validator = required(baseValidator);
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { name: "John", age: 30, active: true };
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error missing age and active
    const wrong: Schema = { name: "John" };
    expect(wrong.name).toBe("John");
  });
});
