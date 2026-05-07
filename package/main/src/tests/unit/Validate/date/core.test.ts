import { date } from "@/Validate/date";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("date validation", () => {
  it("accepts valid Date instances", () => {
    const validator = date();
    expect(validator(new Date(2026, 4, 7)).validate).toBe(true);
    expect(validator(new Date()).validate).toBe(true);
  });

  it("rejects invalid Date instances", () => {
    const validator = date();
    expect(validator(new Date("not a date")).validate).toBe(false);
  });

  it("rejects non-Date values", () => {
    const validator = date();
    // @ts-expect-error string is not Date
    expect(validator("2026-05-07").validate).toBe(false);
    // @ts-expect-error number is not Date
    expect(validator(0).validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = date("must be a date");
    // @ts-expect-error string is not Date
    expect(validator("nope").message).toBe("must be a date");
  });

  it("composes inside object()", () => {
    const validator = object({
      createdAt: date(),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      createdAt: new Date(),
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(date(), string());
    expect(validator(new Date()).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
    // @ts-expect-error number is not in the union
    expect(validator(42).validate).toBe(false);
  });

  it("composes with intersection", () => {
    const validator = intersection(
      object({ createdAt: date() }),
      object({ id: number() }),
    );
    expect(validator({ createdAt: new Date(), id: 1 }).validate).toBe(true);
  });

  it("composes with nullable", () => {
    const validator = nullable(date());
    expect(validator(new Date()).validate).toBe(true);
    expect(validator(null).validate).toBe(true);
  });

  it("composes with optional", () => {
    const validator = optional(date());
    expect(validator(new Date()).validate).toBe(true);
    expect(validator(undefined).validate).toBe(true);
  });

  it("infers Date via SchemaToInterface", () => {
    const validator = date();
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = new Date();
    expect(validator(value).validate).toBe(true);
    // @ts-expect-error string is not Date
    const wrong: Schema = "2026-05-07";
    expect(typeof wrong).toBe("string");
  });

  it("infers nullable Date inside object via SchemaToInterface", () => {
    const validator = object({
      name: string(),
      createdAt: nullable(date()),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = { name: "John", createdAt: null };
    const validWithDate: Schema = { name: "John", createdAt: new Date() };
    expect(validator(valid).validate).toBe(true);
    expect(validator(validWithDate).validate).toBe(true);
  });
});
