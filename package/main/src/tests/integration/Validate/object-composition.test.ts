import { arrayOf } from "@/Validate/array/arrayOf";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import { oneOf } from "@/Validate/string/oneOf";
import type { SchemaToInterface } from "@/Validate/type";

/**
 * Integration tests covering compositions of object/union/intersection
 * validators with primitive and oneOf validators. Exercises both runtime
 * validation and type inference through SchemaToInterface.
 */
describe("Integration: object/union/intersection composition", () => {
  it("validates a deeply nested object structure (3 levels)", () => {
    const validator = object({
      user: object({
        profile: object({
          name: string(),
          age: number(),
        }),
      }),
    });
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = {
      user: { profile: { name: "John", age: 30 } },
    };
    expect(validator(valid).validate).toBe(true);
    expect(
      // @ts-expect-error inner age has the wrong type
      validator({ user: { profile: { name: "x", age: "30" } } }).validate,
    ).toBe(false);
  });

  it("validates an array of discriminated union objects", () => {
    const validator = arrayOf(
      union(
        object({ kind: oneOf(["text"]), value: string() }),
        object({ kind: oneOf(["num"]), value: number() }),
      ),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = [
      { kind: "text", value: "hello" },
      { kind: "num", value: 42 },
    ];
    expect(validator(valid).validate).toBe(true);
    expect(
      validator([
        // @ts-expect-error value type does not match kind
        { kind: "text", value: 1 },
      ]).validate,
    ).toBe(false);
  });

  it("validates an intersection of two object validators", () => {
    const named = object({ name: string() });
    const aged = object({ age: number() });
    const validator = intersection(named, aged);
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = { name: "Jane", age: 25 };
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error age is missing
    expect(validator({ name: "Jane" }).validate).toBe(false);
  });

  it("validates a union of two intersections", () => {
    const left = intersection(
      object({ kind: oneOf(["a"]) }),
      object({ value: string() }),
    );
    const right = intersection(
      object({ kind: oneOf(["b"]) }),
      object({ value: number() }),
    );
    const validator = union(left, right);
    expect(validator({ kind: "a", value: "hello" }).validate).toBe(true);
    expect(validator({ kind: "b", value: 42 }).validate).toBe(true);
    // @ts-expect-error kind=a expects string value
    expect(validator({ kind: "a", value: 1 }).validate).toBe(false);
  });

  it("validates an object whose property is a union, then intersected with another object", () => {
    const validator = intersection(
      object({
        body: union(object({ name: string() }), object({ id: number() })),
      }),
      object({ extra: string() }),
    );
    expect(validator({ body: { name: "John" }, extra: "tag" }).validate).toBe(
      true,
    );
    expect(validator({ body: { id: 1 }, extra: "tag" }).validate).toBe(true);
    // @ts-expect-error extra is missing
    expect(validator({ body: { name: "John" } }).validate).toBe(false);
  });

  it("validates a 3-way union mixing primitives and an object", () => {
    const validator = union(string(), number(), object({ tag: string() }));
    expect(validator("hi").validate).toBe(true);
    expect(validator(7).validate).toBe(true);
    expect(validator({ tag: "x" }).validate).toBe(true);
    // @ts-expect-error boolean is not part of the union
    expect(validator(true).validate).toBe(false);
  });

  it("validates a 3-way intersection of single-property objects", () => {
    const validator = intersection(
      object({ a: string() }),
      object({ b: number() }),
      object({ c: oneOf(["x", "y"]) }),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = { a: "hello", b: 42, c: "x" };
    expect(validator(valid).validate).toBe(true);
    expect(
      // @ts-expect-error c is outside the literal union
      validator({ a: "hello", b: 42, c: "z" }).validate,
    ).toBe(false);
  });

  it("validates an object whose property is a union of nested objects", () => {
    const validator = object({
      payload: union(
        object({ tag: oneOf(["A"]), values: arrayOf(number()) }),
        object({ tag: oneOf(["B"]), labels: arrayOf(string()) }),
      ),
    });
    expect(
      validator({ payload: { tag: "A", values: [1, 2, 3] } }).validate,
    ).toBe(true);
    expect(
      validator({ payload: { tag: "B", labels: ["x", "y"] } }).validate,
    ).toBe(true);
    expect(
      // @ts-expect-error tag and key combination does not match either branch
      validator({ payload: { tag: "A", labels: ["x"] } }).validate,
    ).toBe(false);
  });

  it("merges literal unions when combining oneOf inside union", () => {
    const left = oneOf(["red", "green"]);
    const right = oneOf(["blue", "yellow"]);
    const validator = union(left, right);
    expect(validator("red").validate).toBe(true);
    expect(validator("yellow").validate).toBe(true);
    // @ts-expect-error not part of the merged union
    expect(validator("purple").validate).toBe(false);
  });

  it("preserves the literal union for an intersected object property", () => {
    const validator = intersection(
      object({ shape: oneOf(["circle", "square"]) }),
      object({ size: number() }),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = { shape: "circle", size: 10 };
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error shape outside literal union
    expect(validator({ shape: "triangle", size: 10 }).validate).toBe(false);
  });
});
