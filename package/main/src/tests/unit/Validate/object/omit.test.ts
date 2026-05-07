import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { omit_ } from "@/Validate/object/omit";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("omit_", () => {
  const baseValidator = object({
    name: string(),
    age: number(),
    active: boolean(),
  });

  it("creates a validator covering only the remaining keys", () => {
    const validator = omit_(baseValidator, ["active"]);
    const valid: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("ignores keys outside the omitted set", () => {
    const validator = omit_(baseValidator, ["age", "active"]);
    const valid: ReturnType<typeof validator>["type"] = { name: "John" };
    expect(validator(valid).validate).toBe(true);
  });

  it("rejects values failing remaining validators", () => {
    const validator = omit_(baseValidator, ["active"]);
    // @ts-expect-error wrong type for age
    expect(validator({ name: "John", age: "30" }).validate).toBe(false);
  });

  it("composes with union", () => {
    const validator = union(omit_(baseValidator, ["active"]), string());
    expect(validator({ name: "John", age: 30 }).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection", () => {
    const validator = intersection(
      omit_(baseValidator, ["active"]),
      object({ active: boolean() }),
    );
    expect(validator({ name: "John", age: 30, active: true }).validate).toBe(
      true,
    );
  });

  it("composes with nullable and optional", () => {
    const nullableOmit = nullable(omit_(baseValidator, ["active"]));
    expect(nullableOmit(null).validate).toBe(true);
    expect(nullableOmit({ name: "John", age: 30 }).validate).toBe(true);

    const optionalOmit = optional(omit_(baseValidator, ["active"]));
    expect(optionalOmit(undefined).validate).toBe(true);
    expect(optionalOmit({ name: "John", age: 30 }).validate).toBe(true);
  });

  it("infers the omitted shape via SchemaToInterface", () => {
    const validator = omit_(baseValidator, ["active"]);
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { name: "John", age: 30 };
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error active is omitted
    const wrong: Schema = { name: "John", age: 30, active: true };
    expect(wrong).toBeDefined();
  });
});
