import { function_ } from "@/Validate/function";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("function_ validation", () => {
  it("accepts function values", () => {
    const validator = function_();
    expect(validator(() => 0).validate).toBe(true);
  });

  it("rejects non-function values", () => {
    const validator = function_();
    // @ts-expect-error string is not function
    expect(validator("nope").validate).toBe(false);
    // @ts-expect-error number is not function
    expect(validator(42).validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = function_(undefined, "must be a function");
    // @ts-expect-error string is not function
    expect(validator("nope").message).toBe("must be a function");
  });

  it("composes inside object()", () => {
    const validator = object({
      handler: function_(),
      name: string(),
    });
    const valid: ReturnType<typeof validator>["type"] = {
      handler: () => 1,
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(function_(), string());
    expect(validator(() => 1).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with intersection of objects", () => {
    const validator = intersection(
      object({ handler: function_() }),
      object({ id: number() }),
    );
    expect(validator({ handler: () => 1, id: 1 }).validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const nullableFn = nullable(function_());
    expect(nullableFn(null).validate).toBe(true);
    expect(nullableFn(() => 1).validate).toBe(true);

    const optionalFn = optional(function_());
    expect(optionalFn(undefined).validate).toBe(true);
    expect(optionalFn(() => 1).validate).toBe(true);
  });

  it("validates input arguments via implement()", () => {
    const validator = function_({
      input: [string(), number()],
      output: number(),
    });
    const wrapped = validator.implement((name, age) => age + name.length);
    expect(wrapped("John", 30)).toBe(34);
    // @ts-expect-error wrong arg type
    expect(() => wrapped(1, 30)).toThrow(TypeError);
    // @ts-expect-error wrong arg type
    expect(() => wrapped("John", "30")).toThrow(TypeError);
  });

  it("validates output via implement()", () => {
    const validator = function_({
      input: [number()],
      output: string(),
    });
    const wrapped = validator.implement((n) => n as unknown as string);
    expect(() => wrapped(1)).toThrow(TypeError);
  });

  it("falls back to a default message when input validation fails without one", () => {
    const validator = function_({
      input: [number()],
      output: number(),
    });
    const wrapped = validator.implement((n) => n);
    // @ts-expect-error string is not number
    expect(() => wrapped("nope")).toThrow(
      "function input at index 0 failed validation",
    );
  });

  it("falls back to a default message when output validation fails without one", () => {
    const validator = function_({
      input: [number()],
      output: number(),
    });
    const wrapped = validator.implement((n) => `${n}` as unknown as number);
    expect(() => wrapped(1)).toThrow("function output failed validation");
  });

  it("throws when implement() receives a non-function with the default message", () => {
    const validator = function_();
    expect(() =>
      // @ts-expect-error string is not function
      validator.implement("not a function"),
    ).toThrow("value is not a function");
  });

  it("throws when implement() receives a non-function with a custom message", () => {
    const validator = function_(undefined, "must be callable");
    expect(() =>
      // @ts-expect-error string is not function
      validator.implement("not a function"),
    ).toThrow("must be callable");
  });

  it("skips input checks when implement() runs with no input schema", () => {
    const validator = function_({ output: number() });
    const wrapped = validator.implement(() => 1);
    expect(wrapped()).toBe(1);
  });

  it("skips output checks when implement() runs with no output schema", () => {
    const validator = function_({ input: [number()] });
    const wrapped = validator.implement((n) => n);
    expect(wrapped(2)).toBe(2);
  });

  it("returns the value untouched when implement() runs with no schema", () => {
    const validator = function_();
    const wrapped = validator.implement((value: number) => value * 2);
    expect(wrapped(3)).toBe(6);
  });

  it("returns a wrapper that does not change call results", () => {
    const validator = function_({
      input: [number()],
      output: number(),
    });
    const wrapped = validator.implement((n) => n * 2);
    expect(wrapped(2)).toBe(4);
    expect(wrapped(3)).toBe(6);
  });

  it("infers the function signature via SchemaToInterface", () => {
    const validator = function_({
      input: [string(), number()],
      output: number(),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const fn: Schema = (name, age) => age + name.length;
    expect(fn("John", 30)).toBe(34);
  });
});
