import { arrayOf } from "@/Validate/array/arrayOf";
import { literal } from "@/Validate/literal";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("literal", () => {
  it("accepts only the given string", () => {
    const validator = literal("admin");
    expect(validator("admin")).toEqual({
      validate: true,
      message: "",
      type: "admin",
    });
    // @ts-expect-error "user" is not the literal
    expect(validator("user").validate).toBe(false);
  });

  it("accepts only the given number", () => {
    const validator = literal(42);
    expect(validator(42).validate).toBe(true);
    // @ts-expect-error 43 is not the literal
    expect(validator(43).validate).toBe(false);
    // @ts-expect-error "42" is not the literal
    expect(validator("42").validate).toBe(false);
  });

  it("accepts only the given boolean", () => {
    const validator = literal(true);
    expect(validator(true).validate).toBe(true);
    // @ts-expect-error false is not the literal
    expect(validator(false).validate).toBe(false);
    // @ts-expect-error 1 is not the literal
    expect(validator(1).validate).toBe(false);
  });

  it("accepts only the given bigint", () => {
    const validator = literal(10n);
    expect(validator(10n).validate).toBe(true);
    // @ts-expect-error 10 is a number, not a bigint
    expect(validator(10).validate).toBe(false);
  });

  it("accepts null and undefined literals", () => {
    const nullValidator = literal(null);
    expect(nullValidator(null).validate).toBe(true);
    // @ts-expect-error undefined is not null
    expect(nullValidator(undefined).validate).toBe(false);

    const undefinedValidator = literal(undefined);
    expect(undefinedValidator(undefined).validate).toBe(true);
    // @ts-expect-error null is not undefined
    expect(undefinedValidator(null).validate).toBe(false);
  });

  it("uses strict equality for NaN and signed zero", () => {
    expect(literal(Number.NaN)(Number.NaN).validate).toBe(false);
    expect(literal(0)(-0).validate).toBe(true);
  });

  it("returns the custom message on failure only", () => {
    const validator = literal("a", "must be a");
    // @ts-expect-error "b" is not the literal
    expect(validator("b").message).toBe("must be a");
    expect(validator("a").message).toBe("");
  });

  it("returns an empty message when no message is provided", () => {
    // @ts-expect-error "b" is not the literal
    expect(literal("a")("b").message).toBe("");
  });

  it("composes inside object() and keeps the literal type", () => {
    const validator = object({
      kind: literal("circle"),
      radius: number(),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { kind: "circle", radius: 1 };
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error "square" is not the literal
    const invalid: Schema = { kind: "square", radius: 1 };
    expect(validator(invalid).validate).toBe(false);
  });

  it("keeps a string literal that names a type tag", () => {
    const validator = object({ tag: literal("string") });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { tag: "string" };
    expect(validator(value).validate).toBe(true);
  });

  it("composes with union() and arrayOf()", () => {
    const switchValidator = union(literal("on"), literal("off"));
    // @ts-expect-error "auto" is not part of the union
    expect(switchValidator("auto").validate).toBe(false);
    const validator = arrayOf(switchValidator);
    expect(validator(["on", "off"]).validate).toBe(true);
    expect(validator(["on", "auto"]).validate).toBe(false);
  });

  it("composes with other validators inside union()", () => {
    const validator = union(literal(0), string());
    expect(validator(0).validate).toBe(true);
    expect(validator("x").validate).toBe(true);
  });

  it("exposes Standard Schema V1 validation", () => {
    const validator = literal("a", "must be a");
    expect(validator["~standard"].validate("a")).toEqual({ value: "a" });
    expect(validator["~standard"].validate("b")).toEqual({
      issues: [{ message: "must be a" }],
    });
  });
});
