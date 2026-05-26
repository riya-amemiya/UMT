import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { setOf } from "@/Validate/set";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("set validation", () => {
  it("accepts a Set without item validator", () => {
    const validator = setOf();
    expect(validator(new Set()).validate).toBe(true);
    expect(validator(new Set([1, 2])).validate).toBe(true);
  });

  it("rejects non-Set values", () => {
    const validator = setOf();
    // @ts-expect-error array is not Set
    expect(validator([1, 2]).validate).toBe(false);
    // @ts-expect-error object is not Set
    expect(validator({}).validate).toBe(false);
    // @ts-expect-error null is not Set
    expect(validator(null).validate).toBe(false);
  });

  it("validates items with the item validator", () => {
    const validator = setOf(string());
    expect(validator(new Set(["a", "b"])).validate).toBe(true);
    const invalid = new Set([1 as unknown as string]);
    expect(validator(invalid).validate).toBe(false);
  });

  it("returns the configured message when not a Set", () => {
    const validator = setOf(string(), "must be a Set");
    // @ts-expect-error array is not Set
    expect(validator([]).message).toBe("must be a Set");
  });

  it("propagates the failing item message", () => {
    const validator = setOf(string([], "must be string"));
    const invalid = new Set([1 as unknown as string]);
    expect(validator(invalid).message).toBe("must be string");
  });

  it("composes inside object()", () => {
    const validator = object({
      tags: setOf(string()),
      name: string(),
    });
    const valid = {
      tags: new Set(["a", "b"]),
      name: "John",
    } as ReturnType<typeof validator>["type"];
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(setOf(string()), string());
    expect(validator(new Set(["a"])).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ tags: setOf(string()) }),
      object({ id: number() }),
    );
    expect(validator({ tags: new Set(["a"]), id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableSet = nullable(setOf(string()));
    expect(nullableSet(null).validate).toBe(true);
    expect(nullableSet(new Set(["a"])).validate).toBe(true);

    const optionalSet = optional(setOf(string()));
    expect(optionalSet(undefined).validate).toBe(true);
    expect(optionalSet(new Set(["a"])).validate).toBe(true);
  });

  it("infers Set via SchemaToInterface", () => {
    const validator = setOf(string());
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = new Set(["a"]);
    expect(validator(value).validate).toBe(true);
  });

  it("infers Set inside object via SchemaToInterface", () => {
    const validator = object({
      name: string(),
      tags: setOf(string()),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { name: "John", tags: new Set(["a"]) };
    expect(validator(value).validate).toBe(true);
  });
});
