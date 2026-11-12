import { tuple } from "@/Validate/array/tuple";
import { boolean } from "@/Validate/boolean";
import { literal } from "@/Validate/literal";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { optional } from "@/Validate/object/optional";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("tuple validation", () => {
  it("validates each element with the validator at the same index", () => {
    const validator = tuple([string(), number(), boolean()]);
    expect(validator(["a", 1, true])).toEqual({
      validate: true,
      message: "",
      type: ["a", 1, true],
    });
  });

  it("propagates the message of the first failing element", () => {
    const validator = tuple([
      string([], "first must be a string"),
      number([], "second must be a number"),
    ]);
    // @ts-expect-error second element must be a number
    const result = validator(["a", "b"]);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("second must be a number");
    // @ts-expect-error both elements are invalid
    expect(validator([1, "b"]).message).toBe("first must be a string");
  });

  it("rejects arrays whose length differs from the validators", () => {
    const validator = tuple([string(), number()], "must be [string, number]");
    // @ts-expect-error too short
    const shortResult = validator(["a"]);
    expect(shortResult.validate).toBe(false);
    expect(shortResult.message).toBe("must be [string, number]");
    // @ts-expect-error too long
    const longResult = validator(["a", 1, 2]);
    expect(longResult.validate).toBe(false);
    expect(longResult.message).toBe("must be [string, number]");
  });

  it("rejects non-array values", () => {
    const validator = tuple([string()], "not a tuple");
    // @ts-expect-error null is not an array
    expect(validator(null).message).toBe("not a tuple");
    // @ts-expect-error object is not an array
    expect(validator({ 0: "a", length: 1 }).validate).toBe(false);
    // @ts-expect-error string is not an array
    expect(validator("a").validate).toBe(false);
  });

  it("returns an empty message when no message is provided", () => {
    const validator = tuple([string()]);
    // @ts-expect-error too long
    expect(validator(["a", "b"]).message).toBe("");
  });

  it("accepts only the empty array for an empty tuple", () => {
    const validator = tuple([]);
    expect(validator([]).validate).toBe(true);
    // @ts-expect-error too long
    expect(validator([1]).validate).toBe(false);
  });

  it("checks the length before running optional element validators", () => {
    const validator = tuple([string(), optional(number())]);
    expect(validator(["a", undefined]).validate).toBe(true);
    // @ts-expect-error missing second element
    expect(validator(["a"]).validate).toBe(false);
  });

  it("composes inside object() and infers the tuple type", () => {
    const validator = object({
      point: tuple([number(), number()]),
      label: tuple([literal("x"), string()]),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { point: [1, 2], label: ["x", "y"] };
    expect(validator(value).validate).toBe(true);
    const first: number = value.point[0];
    expect(first).toBe(1);
  });

  it("exposes Standard Schema V1 validation", () => {
    const validator = tuple([string()], "bad tuple");
    expect(validator["~standard"].validate(["a"])).toEqual({ value: ["a"] });
    expect(validator["~standard"].validate([])).toEqual({
      issues: [{ message: "bad tuple" }],
    });
  });
});
