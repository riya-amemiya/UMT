import { arrayOf } from "@/Validate/array/arrayOf";
import { function_ } from "@/Validate/function/core";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import { oneOf } from "@/Validate/string/oneOf";
import { templateLiteral } from "@/Validate/templateLiteral/core";

/**
 * Integration tests covering literal validators (oneOf, templateLiteral) and
 * the function_ validator combined with object/union/intersection/arrayOf.
 * Verifies that literal unions flow through compositions and that function_
 * implement() runs nested validators on inputs and outputs.
 */
describe("Integration: literal, template literal, and function composition", () => {
  it("validates an object whose key is a templateLiteral and another is oneOf", () => {
    const validator = object({
      slug: templateLiteral(["item-", number()]),
      status: oneOf(["draft", "published"]),
    });
    expect(validator({ slug: "item-1", status: "draft" }).validate).toBe(true);
    expect(
      // @ts-expect-error slug pattern does not match
      validator({ slug: "thing-1", status: "draft" }).validate,
    ).toBe(false);
    expect(
      // @ts-expect-error status outside literal union
      validator({ slug: "item-1", status: "archived" }).validate,
    ).toBe(false);
  });

  it("validates an array of templateLiteral strings", () => {
    const validator = arrayOf(templateLiteral(["user-", number()]));
    expect(validator(["user-1", "user-2"]).validate).toBe(true);
    // @ts-expect-error second element does not match the template
    expect(validator(["user-1", "admin-1"]).validate).toBe(false);
  });

  it("validates intersection of object containing templateLiteral with another object", () => {
    const validator = intersection(
      object({ id: templateLiteral(["id_", number()]) }),
      object({ label: oneOf(["primary", "secondary"]) }),
    );
    expect(validator({ id: "id_1", label: "primary" }).validate).toBe(true);
    // @ts-expect-error id pattern wrong
    expect(validator({ id: "1", label: "primary" }).validate).toBe(false);
  });

  it("validates a union mixing templateLiteral with oneOf", () => {
    const validator = union(
      templateLiteral(["v", number()]),
      oneOf(["latest", "stable"]),
    );
    expect(validator("v1").validate).toBe(true);
    expect(validator("v10.5").validate).toBe(true);
    expect(validator("latest").validate).toBe(true);
    expect(validator("stable").validate).toBe(true);
    // @ts-expect-error value matches neither branch
    expect(validator("vAlpha").validate).toBe(false);
  });

  it("composes function_ with object input and arrayOf output via implement", () => {
    const validator = function_({
      input: [object({ name: string(), age: number() })],
      output: arrayOf(string()),
    });
    const wrapped = validator.implement((user) => [
      `name:${user.name}`,
      `age:${user.age}`,
    ]);
    expect(wrapped({ name: "John", age: 30 })).toEqual(["name:John", "age:30"]);
    expect(() =>
      // @ts-expect-error input shape wrong
      wrapped({ name: "John" }),
    ).toThrow(TypeError);
  });

  it("composes function_ with union input and templateLiteral output", () => {
    const validator = function_({
      input: [union(string(), number())],
      output: templateLiteral(["wrapped:", string()]),
    });
    const wrapped = validator.implement(
      (value) => `wrapped:${String(value)}` as `wrapped:${string}`,
    );
    expect(wrapped("hello")).toBe("wrapped:hello");
    expect(wrapped(42)).toBe("wrapped:42");
    // @ts-expect-error boolean is not part of the input union
    expect(() => wrapped(true)).toThrow(TypeError);
  });

  it("validates an array of objects, each with a function_ value", () => {
    const validator = arrayOf(
      object({
        name: string(),
        handler: function_({ input: [number()], output: number() }),
      }),
    );
    const valid = [
      { name: "double", handler: (n: number) => n * 2 },
      { name: "negate", handler: (n: number) => -n },
    ];
    expect(validator(valid).validate).toBe(true);
    expect(
      // @ts-expect-error handler must be a function
      validator([{ name: "broken", handler: 1 }]).validate,
    ).toBe(false);
  });

  it("validates a deeply tagged shape using oneOf within nested object", () => {
    const validator = arrayOf(
      object({
        meta: object({
          severity: oneOf(["info", "warn", "error"]),
        }),
        message: string(),
      }),
    );
    expect(
      validator([
        { meta: { severity: "info" }, message: "ok" },
        { meta: { severity: "error" }, message: "boom" },
      ]).validate,
    ).toBe(true);
    expect(
      // @ts-expect-error severity outside literal union
      validator([{ meta: { severity: "fatal" }, message: "x" }]).validate,
    ).toBe(false);
  });

  it("rejects function_ implement output that does not match the schema", () => {
    const validator = function_({
      input: [number()],
      output: oneOf(["even", "odd"]),
    });
    const wrapped = validator.implement((n) => (n % 2 === 0 ? "even" : "odd"));
    expect(wrapped(2)).toBe("even");
    expect(wrapped(3)).toBe("odd");

    const broken = validator.implement(
      () => "unknown" as unknown as "even" | "odd",
    );
    expect(() => broken(1)).toThrow(TypeError);
  });
});
