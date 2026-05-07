import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { pick_ } from "@/Validate/object/pick";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("pick_", () => {
  const baseValidator = object({
    name: string(),
    age: number(),
    active: boolean(),
  });

  it("creates a validator covering only the picked keys", () => {
    const validator = pick_(baseValidator, ["name", "age"]);
    const valid: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("ignores keys outside the picked set", () => {
    const validator = pick_(baseValidator, ["name"]);
    const valid: ReturnType<typeof validator>["type"] = { name: "John" };
    expect(validator(valid).validate).toBe(true);
  });

  it("rejects values failing one of the picked validators", () => {
    const validator = pick_(baseValidator, ["age"]);
    // @ts-expect-error wrong type for age
    expect(validator({ age: "30" }).validate).toBe(false);
  });

  it("throws on unknown picked keys", () => {
    expect(() =>
      // @ts-expect-error unknown key
      pick_(baseValidator, ["unknown"]),
    ).toThrow(TypeError);
  });

  it("composes with union", () => {
    const validator = union(pick_(baseValidator, ["name"]), string());
    expect(validator({ name: "John" }).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection", () => {
    const validator = intersection(
      pick_(baseValidator, ["name"]),
      pick_(baseValidator, ["age"]),
    );
    expect(validator({ name: "John", age: 30 }).validate).toBe(true);
    // @ts-expect-error missing age
    expect(validator({ name: "John" }).validate).toBe(false);
  });

  it("composes with nullable and optional", () => {
    const nullablePick = nullable(pick_(baseValidator, ["name"]));
    expect(nullablePick(null).validate).toBe(true);
    expect(nullablePick({ name: "John" }).validate).toBe(true);

    const optionalPick = optional(pick_(baseValidator, ["name"]));
    expect(optionalPick(undefined).validate).toBe(true);
    expect(optionalPick({ name: "John" }).validate).toBe(true);
  });

  it("infers the picked shape via SchemaToInterface", () => {
    const validator = pick_(baseValidator, ["name", "age"]);
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { name: "John", age: 30 };
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error active not in picked shape
    const wrong: Schema = { name: "John", age: 30, active: true };
    expect(wrong).toBeDefined();
  });
});
