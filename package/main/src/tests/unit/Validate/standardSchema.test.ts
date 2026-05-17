import { any } from "@/Validate/any";
import { arrayOf } from "@/Validate/array";
import { bigint } from "@/Validate/bigint";
import { boolean } from "@/Validate/boolean";
import { date } from "@/Validate/date";
import { file } from "@/Validate/file";
import { function_ } from "@/Validate/function";
import { instanceof_ } from "@/Validate/instanceof";
import { map } from "@/Validate/map";
import { never } from "@/Validate/never";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { set_ } from "@/Validate/set";
import {
  STANDARD_SCHEMA_VENDOR,
  type StandardSchemaV1,
  type StandardSchemaV1FailureResult,
  type StandardSchemaV1SuccessResult,
} from "@/Validate/standardSchema";
import { string } from "@/Validate/string";
import { templateLiteral } from "@/Validate/templateLiteral";
import { unknown } from "@/Validate/unknown";

type SyncResult<O> =
  | StandardSchemaV1SuccessResult<O>
  | StandardSchemaV1FailureResult;

const syncValidate = <I, O>(
  schema: StandardSchemaV1<I, O>,
  value: unknown,
): SyncResult<O> => {
  const result = schema["~standard"].validate(value);
  if (result instanceof Promise) {
    throw new TypeError("Expected synchronous validation result");
  }
  return result;
};

const expectedProps = {
  version: 1,
  vendor: STANDARD_SCHEMA_VENDOR,
  // biome-ignore lint/suspicious/noExplicitAny: matcher is expect.any
  validate: expect.any(Function) as unknown as (value: unknown) => any,
};

describe("Standard Schema V1 compatibility", () => {
  it("string validator exposes the spec interface", () => {
    const validator = string();
    expect(validator["~standard"]).toMatchObject(expectedProps);
    const ok = syncValidate(validator, "hello");
    expect(ok.issues).toBeUndefined();
    expect((ok as StandardSchemaV1SuccessResult<string>).value).toBe("hello");

    const ng = syncValidate(validator, 123);
    expect((ng as StandardSchemaV1FailureResult).issues.length).toBe(1);
  });

  it("number validator surfaces the configured message on failure", () => {
    const validator = number([], "must be a number");
    expect(validator["~standard"]).toMatchObject(expectedProps);
    const result = syncValidate(validator, "not-a-number");
    expect((result as StandardSchemaV1FailureResult).issues[0]?.message).toBe(
      "must be a number",
    );
  });

  it("boolean / bigint / date / file validators implement the spec", () => {
    expect(boolean()["~standard"]).toMatchObject(expectedProps);
    expect(bigint()["~standard"]).toMatchObject(expectedProps);
    expect(date()["~standard"]).toMatchObject(expectedProps);
    expect(file()["~standard"]).toMatchObject(expectedProps);
  });

  it("any / unknown / never validators implement the spec", () => {
    const anyValidator = any();
    const unknownValidator = unknown();
    const neverValidator = never("never message");
    expect(anyValidator["~standard"]).toMatchObject(expectedProps);
    expect(unknownValidator["~standard"]).toMatchObject(expectedProps);
    expect(neverValidator["~standard"]).toMatchObject(expectedProps);

    expect(syncValidate(anyValidator, Symbol("x")).issues).toBeUndefined();
    expect(syncValidate(unknownValidator, null).issues).toBeUndefined();

    const neverResult = syncValidate(neverValidator, 1);
    expect(
      (neverResult as StandardSchemaV1FailureResult).issues[0]?.message,
    ).toBe("never message");
  });

  it("instanceof / arrayOf / set_ / map validators implement the spec", () => {
    class Sample {
      readonly id = 1;
    }
    expect(instanceof_(Sample)["~standard"]).toMatchObject(expectedProps);
    expect(arrayOf(string())["~standard"]).toMatchObject(expectedProps);
    expect(set_(string())["~standard"]).toMatchObject(expectedProps);
    expect(map(string(), number())["~standard"]).toMatchObject(expectedProps);
  });

  it("templateLiteral validator validates input through the spec", () => {
    const validator = templateLiteral(["user-", number()]);
    expect(validator["~standard"]).toMatchObject(expectedProps);
    expect(syncValidate(validator, "user-1").issues).toBeUndefined();
    expect(syncValidate(validator, "admin-1").issues?.length).toBe(1);
  });

  it("object validator implements the spec and validates payloads", () => {
    const validator = object({
      name: string(),
      age: number(),
    });
    expect(validator["~standard"]).toMatchObject(expectedProps);
    const ok = syncValidate(validator, { name: "Alice", age: 30 });
    expect(ok.issues).toBeUndefined();
    const ng = syncValidate(validator, { name: "Alice" });
    expect(ng.issues?.length).toBe(1);
  });

  it("union / intersection / optional / nullable validators implement the spec", () => {
    const stringOrNumber = union(string(), number());
    const both = intersection(object({ a: string() }), object({ b: number() }));
    const maybeString = optional(string());
    const stringOrNull = nullable(string());

    expect(stringOrNumber["~standard"]).toMatchObject(expectedProps);
    expect(both["~standard"]).toMatchObject(expectedProps);
    expect(maybeString["~standard"]).toMatchObject(expectedProps);
    expect(stringOrNull["~standard"]).toMatchObject(expectedProps);

    expect(syncValidate(stringOrNumber, "x").issues).toBeUndefined();
    expect(syncValidate(stringOrNumber, 1).issues).toBeUndefined();
    expect(syncValidate(stringOrNumber, true).issues?.length).toBe(1);

    expect(syncValidate(both, { a: "x", b: 1 }).issues).toBeUndefined();
    expect(syncValidate(both, { a: "x" }).issues?.length).toBe(1);

    expect(syncValidate(maybeString, undefined).issues).toBeUndefined();
    expect(syncValidate(maybeString, "x").issues).toBeUndefined();

    expect(syncValidate(stringOrNull, null).issues).toBeUndefined();
    expect(syncValidate(stringOrNull, "x").issues).toBeUndefined();
  });

  it("function_ validator implements the spec", () => {
    const validator = function_({
      input: [string()],
      output: number(),
    });
    expect(validator["~standard"]).toMatchObject(expectedProps);
    expect(syncValidate(validator, () => 1).issues).toBeUndefined();
    expect(syncValidate(validator, "not a function").issues?.length).toBe(1);
  });

  it("emits a default issue message when the validator has none", () => {
    const validator = string();
    const result = syncValidate(validator, 123);
    expect((result as StandardSchemaV1FailureResult).issues[0]?.message).toBe(
      "Validation failed",
    );
  });
});
