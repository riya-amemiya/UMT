import { bigint } from "@/Validate/bigint";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("bigint validation", () => {
  it("accepts bigint values", () => {
    const validator = bigint();
    expect(validator(BigInt(0)).validate).toBe(true);
    expect(validator(BigInt(123)).validate).toBe(true);
    expect(validator(BigInt("-9007199254740993")).validate).toBe(true);
  });

  it("rejects non-bigint values", () => {
    const validator = bigint();
    // @ts-expect-error number is not bigint
    expect(validator(0).validate).toBe(false);
    // @ts-expect-error string is not bigint
    expect(validator("0").validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = bigint([], "must be a bigint");
    // @ts-expect-error number is not bigint
    expect(validator(1).message).toBe("must be a bigint");
  });

  it("applies validation rules", () => {
    const positive = {
      type: "bigint" as const,
      message: "must be positive",
      validate: (value: bigint) => value > BigInt(0),
    };
    const validator = bigint([positive]);
    expect(validator(BigInt(1)).validate).toBe(true);
    expect(validator(BigInt(0)).validate).toBe(false);
    expect(validator(BigInt(-1)).message).toBe("must be positive");
  });

  it("propagates the bigint type tag through union", () => {
    const validator = union(bigint(), string());
    expect(validator(BigInt(1)).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
    // @ts-expect-error boolean is not in the union
    expect(validator(true).validate).toBe(false);
  });

  it("composes inside intersection of objects", () => {
    const validator = intersection(
      object({ id: bigint() }),
      object({ name: string() }),
    );
    expect(validator({ id: BigInt(1), name: "John" }).validate).toBe(true);
    // @ts-expect-error missing name
    expect(validator({ id: BigInt(1) }).validate).toBe(false);
  });

  it("works inside object()", () => {
    const validator = object({
      id: bigint(),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      id: BigInt(1),
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with nullable", () => {
    const validator = nullable(bigint());
    expect(validator(BigInt(1)).validate).toBe(true);
    expect(validator(null).validate).toBe(true);
    // @ts-expect-error number is not bigint or null
    expect(validator(1).validate).toBe(false);
  });

  it("composes with optional", () => {
    const validator = optional(bigint());
    expect(validator(BigInt(1)).validate).toBe(true);
    expect(validator(undefined).validate).toBe(true);
    // @ts-expect-error number is not bigint or undefined
    expect(validator(1).validate).toBe(false);
  });

  it("infers bigint via SchemaToInterface", () => {
    const validator = bigint();
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = BigInt(42);
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error number not assignable to bigint
    const wrong: Schema = 1;
    expect(typeof wrong).toBe("number");
  });

  it("infers nullable bigint via SchemaToInterface", () => {
    const validator = nullable(bigint());
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = BigInt(42);
    const nullValue: Schema = null;
    expect(validator(value).validate).toBe(true);
    expect(validator(nullValue).validate).toBe(true);
  });

  it("infers bigint inside object via SchemaToInterface", () => {
    const validator = object({ id: bigint(), name: string() });
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = { id: BigInt(1), name: "John" };
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error id must be bigint
    const wrong: Schema = { id: 1, name: "John" };
    expect(wrong.name).toBe("John");
  });
});
