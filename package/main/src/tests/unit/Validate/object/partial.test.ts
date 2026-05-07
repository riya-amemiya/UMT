import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { partial } from "@/Validate/object/partial";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("partial", () => {
  const baseValidator = object({
    name: string(),
    age: number(),
    active: boolean(),
  });

  it("makes every property optional", () => {
    const validator = partial(baseValidator);
    const empty: ReturnType<typeof validator>["type"] = {};
    const partialData: ReturnType<typeof validator>["type"] = { name: "John" };
    const full: ReturnType<typeof validator>["type"] = {
      name: "John",
      age: 30,
      active: true,
    };
    expect(validator(empty).validate).toBe(true);
    expect(validator(partialData).validate).toBe(true);
    expect(validator(full).validate).toBe(true);
  });

  it("validates supplied properties against their original validators", () => {
    const validator = partial(baseValidator);
    // @ts-expect-error wrong type for age
    expect(validator({ age: "30" }).validate).toBe(false);
  });

  it("is idempotent across multiple wrappings", () => {
    const validator = partial(partial(baseValidator));
    expect(validator({}).validate).toBe(true);
    // @ts-expect-error wrong type for age
    expect(validator({ age: "30" }).validate).toBe(false);
  });

  it("composes with union", () => {
    const validator = union(partial(baseValidator), string());
    expect(validator({}).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection", () => {
    const validator = intersection(
      partial(baseValidator),
      object({ id: number() }),
    );
    expect(validator({ id: 1 }).validate).toBe(true);
    expect(
      validator({ id: 1, name: "John", age: 30, active: true }).validate,
    ).toBe(true);
  });

  it("composes with nullable", () => {
    const validator = nullable(partial(baseValidator));
    expect(validator(null).validate).toBe(true);
    expect(validator({}).validate).toBe(true);
    expect(validator({ name: "John" }).validate).toBe(true);
  });

  it("composes with optional", () => {
    const validator = optional(partial(baseValidator));
    expect(validator(undefined).validate).toBe(true);
    expect(validator({}).validate).toBe(true);
  });

  it("infers Partial<T> via SchemaToInterface", () => {
    const validator = partial(baseValidator);
    type Schema = SchemaToInterface<typeof validator>;
    const empty: Schema = {};
    const partialData: Schema = { name: "John" };
    const full: Schema = { name: "John", age: 30, active: true };
    expect(validator(empty).validate).toBe(true);
    expect(validator(partialData).validate).toBe(true);
    expect(validator(full).validate).toBe(true);
  });
});
