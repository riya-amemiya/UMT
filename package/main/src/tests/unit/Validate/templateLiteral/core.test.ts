import { bigint } from "@/Validate/bigint";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import { templateLiteral } from "@/Validate/templateLiteral";
import type { SchemaToInterface } from "@/Validate/type";

describe("templateLiteral validation", () => {
  it("accepts strings matching the template", () => {
    const validator = templateLiteral(["hello, ", string(), "!"]);
    expect(validator("hello, world!").validate).toBe(true);
    expect(validator("hello, John!").validate).toBe(true);
  });

  it("rejects strings that do not match the template", () => {
    const validator = templateLiteral(["hello, ", string(), "!"]);
    // @ts-expect-error literal does not match template literal type
    expect(validator("hi, world!").validate).toBe(false);
    // @ts-expect-error missing trailing punctuation
    expect(validator("hello, world").validate).toBe(false);
  });

  it("rejects non-string values", () => {
    const validator = templateLiteral(["hello, ", string(), "!"]);
    // @ts-expect-error number not assignable to template literal type
    expect(validator(42).validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = templateLiteral(
      ["user-", number()],
      "must be user-<number>",
    );
    // @ts-expect-error "abc" is not a number suffix
    expect(validator("user-abc").message).toBe("must be user-<number>");
  });

  it("supports number validators", () => {
    const validator = templateLiteral(["item-", number()]);
    expect(validator("item-1").validate).toBe(true);
    expect(validator("item-100").validate).toBe(true);
    expect(validator("item-1.5").validate).toBe(true);
    // @ts-expect-error "abc" is not a number suffix
    expect(validator("item-abc").validate).toBe(false);
  });

  it("supports boolean validators", () => {
    const validator = templateLiteral(["flag-", boolean()]);
    expect(validator("flag-true").validate).toBe(true);
    expect(validator("flag-false").validate).toBe(true);
    // @ts-expect-error "yes" is not boolean
    expect(validator("flag-yes").validate).toBe(false);
  });

  it("supports bigint validators", () => {
    const validator = templateLiteral(["big-", bigint()]);
    expect(validator("big-1").validate).toBe(true);
    expect(validator("big-100").validate).toBe(true);
    // @ts-expect-error "abc" is not a bigint suffix
    expect(validator("big-abc").validate).toBe(false);
  });

  it("escapes special regex characters in literal parts", () => {
    const validator = templateLiteral(["[$]", string(), "."]);
    expect(validator("[$]anything.").validate).toBe(true);
    // @ts-expect-error literal does not match template literal type
    expect(validator("xanythingy").validate).toBe(false);
  });

  it("falls back to a permissive pattern for validators with no detectable tag", () => {
    const customValidator = (() => ({
      validate: true,
      message: "",
      type: "custom" as const,
    })) as (value: string) => {
      validate: boolean;
      message: string;
      type: "custom";
    };
    const validator = templateLiteral(["prefix-", customValidator]);
    expect(validator("prefix-anything" as never).validate).toBe(true);
    expect(validator("noprefix" as never).validate).toBe(false);
  });

  it("composes inside object()", () => {
    const validator = object({
      slug: templateLiteral(["user-", number()]),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      slug: "user-1",
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(templateLiteral(["user-", number()]), string());
    expect(validator("user-1").validate).toBe(true);
    expect(validator("plain").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ slug: templateLiteral(["user-", number()]) }),
      object({ id: number() }),
    );
    expect(validator({ slug: "user-1", id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableTpl = nullable(templateLiteral(["user-", number()]));
    expect(nullableTpl(null).validate).toBe(true);
    expect(nullableTpl("user-1").validate).toBe(true);

    const optionalTpl = optional(templateLiteral(["user-", number()]));
    expect(optionalTpl(undefined).validate).toBe(true);
    expect(optionalTpl("user-1").validate).toBe(true);
  });

  it("infers a template literal type via SchemaToInterface", () => {
    const validator = templateLiteral(["hello, ", string(), "!"]);
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = "hello, world!";
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error literal must respect the template literal type
    const wrong: Schema = "wrong";
    expect(typeof wrong).toBe("string");
  });
});
