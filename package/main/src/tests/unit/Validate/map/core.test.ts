import { map } from "@/Validate/map";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("map validation", () => {
  it("accepts a Map without entry validators", () => {
    const validator = map();
    expect(validator(new Map()).validate).toBe(true);
    expect(validator(new Map([["a", 1]])).validate).toBe(true);
  });

  it("rejects non-Map values", () => {
    const validator = map();
    // @ts-expect-error object is not Map
    expect(validator({}).validate).toBe(false);
    // @ts-expect-error array is not Map
    expect(validator([]).validate).toBe(false);
    // @ts-expect-error null is not Map
    expect(validator(null).validate).toBe(false);
  });

  it("validates keys with the key validator", () => {
    const validator = map(string(), number());
    const valid = new Map<string, number>([
      ["a", 1],
      ["b", 2],
    ]);
    expect(validator(valid).validate).toBe(true);

    const invalidKey = new Map([[1 as unknown as string, 1]]);
    expect(validator(invalidKey).validate).toBe(false);
  });

  it("validates values with the value validator", () => {
    const validator = map(string(), number());
    const invalidValue = new Map([["a", "not a number" as unknown as number]]);
    expect(validator(invalidValue).validate).toBe(false);
  });

  it("returns the configured message when not a Map", () => {
    const validator = map(string(), number(), "must be a Map");
    // @ts-expect-error object is not Map
    expect(validator({}).message).toBe("must be a Map");
  });

  it("propagates the failing entry message", () => {
    const validator = map(
      string([], "key must be string"),
      number([], "value must be number"),
    );
    const invalidValue = new Map([["a", "x" as unknown as number]]);
    expect(validator(invalidValue).message).toBe("value must be number");
  });

  it("composes inside object()", () => {
    const validator = object({
      data: map(string(), number()),
      name: string(),
    });
    const valid = {
      data: new Map([["a", 1]]),
      name: "John",
    } as ReturnType<typeof validator>["type"];
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(map(string(), number()), string());
    expect(validator(new Map([["a", 1]])).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ data: map(string(), number()) }),
      object({ id: number() }),
    );
    expect(validator({ data: new Map([["a", 1]]), id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableMap = nullable(map(string(), number()));
    expect(nullableMap(null).validate).toBe(true);
    expect(nullableMap(new Map([["a", 1]])).validate).toBe(true);

    const optionalMap = optional(map(string(), number()));
    expect(optionalMap(undefined).validate).toBe(true);
    expect(optionalMap(new Map([["a", 1]])).validate).toBe(true);
  });

  it("infers Map via SchemaToInterface", () => {
    const validator = map(string(), number());
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = new Map([["a", 1]]);
    expect(validator(value).validate).toBe(true);
  });

  it("infers Map inside object via SchemaToInterface", () => {
    const validator = object({
      name: string(),
      data: map(string(), number()),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = {
      name: "John",
      data: new Map([["a", 1]]),
    };
    expect(validator(value).validate).toBe(true);
  });
});
