import { never } from "@/Validate/never";
import { object } from "@/Validate/object/core";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("never validation", () => {
  it("rejects every value", () => {
    const validator = never();
    expect(validator(0 as never).validate).toBe(false);
    expect(validator("" as never).validate).toBe(false);
    expect(validator(null as never).validate).toBe(false);
    expect(validator(undefined as never).validate).toBe(false);
    expect(validator({} as never).validate).toBe(false);
  });

  it("exposes the never tag through the type field", () => {
    const validator = never();
    expect(validator(42 as never).type).toBe("never");
  });

  it("returns the configured message on failure", () => {
    const validator = never("must never be set");
    expect(validator(0 as never).message).toBe("must never be set");
  });

  it("composes inside object()", () => {
    const validator = object({
      forbidden: never(),
      name: string(),
    });
    expect(
      validator({ forbidden: 0 as never, name: "John" } as never).validate,
    ).toBe(false);
  });

  it("composes with union (only the other branch can ever pass)", () => {
    const validator = union(string(), never());
    expect(validator("hello").validate).toBe(true);
    // @ts-expect-error number is not in the union
    expect(validator(42).validate).toBe(false);
  });

  it("infers never via SchemaToInterface", () => {
    const validator = never();
    type Schema = SchemaToInterface<typeof validator>;
    const accept = (value: Schema) => value;
    expect(typeof accept).toBe("function");
  });
});
