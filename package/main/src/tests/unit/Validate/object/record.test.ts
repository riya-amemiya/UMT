import { arrayOf } from "@/Validate/array/arrayOf";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { record } from "@/Validate/object/record";
import { minLength, oneOf, string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("record validation", () => {
  it("accepts an empty object", () => {
    const validator = record(string(), number());
    expect(validator({})).toEqual({ validate: true, message: "", type: {} });
  });

  it("validates every key and value", () => {
    const validator = record(string(), number());
    expect(validator({ a: 1, b: 2 }).validate).toBe(true);
  });

  it("propagates the failing value message", () => {
    const validator = record(string(), number([], "value must be a number"));
    // @ts-expect-error value must be a number
    const result = validator({ a: 1, b: "x" });
    expect(result.validate).toBe(false);
    expect(result.message).toBe("value must be a number");
  });

  it("propagates the failing key message", () => {
    const validator = record(
      string([minLength(2, "key too short")]),
      number([], "value must be a number"),
    );
    const result = validator({ ab: 1, c: 2 });
    expect(result.validate).toBe(false);
    expect(result.message).toBe("key too short");
  });

  it("checks the key before its value", () => {
    const validator = record(
      string([minLength(2, "key too short")]),
      number([], "value must be a number"),
    );
    // @ts-expect-error value must be a number
    expect(validator({ c: "x" }).message).toBe("key too short");
  });

  it("restricts keys with oneOf()", () => {
    const validator = record(oneOf(["en", "ja"], "unknown locale"), string());
    expect(validator({ en: "Hello", ja: "Konnichiwa" }).validate).toBe(true);
    const invalid = { en: "Hello", fr: "Bonjour" } as unknown as Record<
      "en" | "ja",
      string
    >;
    expect(validator(invalid).message).toBe("unknown locale");
  });

  it("rejects non-dictionary values", () => {
    const validator = record(string(), number(), "must be a record");
    // @ts-expect-error null is not a record
    expect(validator(null).message).toBe("must be a record");
    // @ts-expect-error array is not a record
    expect(validator([1, 2]).validate).toBe(false);
    // @ts-expect-error string is not a record
    expect(validator("a").validate).toBe(false);
  });

  it("returns an empty message when no message is provided", () => {
    const validator = record(string(), number());
    // @ts-expect-error null is not a record
    expect(validator(null).message).toBe("");
  });

  it("composes inside object() and arrayOf()", () => {
    const validator = object({
      scores: record(string(), arrayOf(number())),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { scores: { alice: [1, 2], bob: [] } };
    expect(validator(value).validate).toBe(true);
    const { alice } = value.scores;
    expect(alice).toEqual([1, 2]);
  });

  it("exposes Standard Schema V1 validation", () => {
    const validator = record(string(), number(), "bad record");
    expect(validator["~standard"].validate({ a: 1 })).toEqual({
      value: { a: 1 },
    });
    expect(validator["~standard"].validate(1)).toEqual({
      issues: [{ message: "bad record" }],
    });
  });
});
