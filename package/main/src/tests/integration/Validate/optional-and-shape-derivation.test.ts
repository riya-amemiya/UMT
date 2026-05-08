import { arrayOf } from "@/Validate/array/arrayOf";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { omit_ } from "@/Validate/object/omit";
import { optional } from "@/Validate/object/optional";
import { partial } from "@/Validate/object/partial";
import { pick_ } from "@/Validate/object/pick";
import { required } from "@/Validate/object/required";
import { string } from "@/Validate/string";

/**
 * Integration tests covering optional/nullable wrappers and the shape
 * derivation helpers (partial, required, pick, omit). Verifies that the
 * derived validators still compose with arrayOf, intersection, and nested
 * object validators while behaving as expected at runtime.
 */
describe("Integration: optional, nullable, and shape derivation", () => {
  it("accepts null when nullable wraps a string validator", () => {
    const validator = nullable(string());
    expect(validator(null).validate).toBe(true);
    expect(validator("hi").validate).toBe(true);
    // @ts-expect-error number is not allowed
    expect(validator(1).validate).toBe(false);
  });

  it("accepts undefined or null when optional wraps nullable", () => {
    const validator = optional(nullable(string()));
    expect(validator(undefined).validate).toBe(true);
    expect(validator(null).validate).toBe(true);
    expect(validator("hi").validate).toBe(true);
    // @ts-expect-error number is not allowed
    expect(validator(1).validate).toBe(false);
  });

  it("validates an array of optional strings", () => {
    const validator = arrayOf(optional(string()));
    expect(validator([undefined, "a", "b"]).validate).toBe(true);
    // @ts-expect-error number element is not allowed
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("validates an array of nullable strings", () => {
    const validator = arrayOf(nullable(string()));
    expect(validator([null, "a", "b"]).validate).toBe(true);
    // @ts-expect-error number element is not allowed
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("validates an object with both optional and nullable properties", () => {
    const validator = object({
      name: string(),
      age: optional(number()),
      bio: nullable(string()),
    });
    expect(validator({ name: "x", bio: null }).validate).toBe(true);
    expect(validator({ name: "x", age: 1, bio: "hi" }).validate).toBe(true);
    expect(
      // @ts-expect-error name is required
      validator({ bio: null }).validate,
    ).toBe(false);
  });

  it("makes every property optional via partial and reverses with required", () => {
    const base = object({ name: string(), age: number() });
    const lax = partial(base);
    expect(lax({}).validate).toBe(true);
    expect(lax({ name: "x" }).validate).toBe(true);

    const strict = required(lax);
    expect(strict({ name: "x", age: 1 }).validate).toBe(true);
    // @ts-expect-error required brings back age
    expect(strict({ name: "x" }).validate).toBe(false);
  });

  it("derives a smaller validator via pick", () => {
    const base = object({ id: number(), name: string(), age: number() });
    const onlyId = pick_(base, ["id"]);
    expect(onlyId({ id: 1 }).validate).toBe(true);
    // The picked validator only checks `id`, so unknown extra keys pass through.
    expect(onlyId({ id: 2, name: "x" } as { id: number }).validate).toBe(true);
  });

  it("derives a smaller validator via omit", () => {
    const base = object({ id: number(), name: string(), age: number() });
    const noAge = omit_(base, ["age"]);
    expect(noAge({ id: 1, name: "x" }).validate).toBe(true);
    expect(
      // @ts-expect-error name expected to be string
      noAge({ id: 1, name: 1 }).validate,
    ).toBe(false);
  });

  it("composes pick + partial + required to surgically toggle optionality", () => {
    const base = object({ id: number(), name: string(), age: number() });
    const lax = partial(pick_(base, ["name", "age"]));
    expect(lax({}).validate).toBe(true);

    const strict = required(lax);
    // @ts-expect-error required brings back age and name
    expect(strict({ name: "x" }).validate).toBe(false);
    expect(strict({ name: "x", age: 1 }).validate).toBe(true);
  });

  it("validates an intersection of partial and required objects", () => {
    const base = object({ name: string(), age: number() });
    const validator = intersection(partial(base), object({ id: number() }));
    expect(validator({ id: 1 }).validate).toBe(true);
    expect(validator({ id: 1, name: "x", age: 2 }).validate).toBe(true);
    expect(
      // @ts-expect-error id is required by the right side
      validator({ name: "x" }).validate,
    ).toBe(false);
  });

  it("validates nested optional arrayOf inside an object", () => {
    const validator = object({
      title: string(),
      tags: optional(arrayOf(string())),
    });
    expect(validator({ title: "post" }).validate).toBe(true);
    expect(validator({ title: "post", tags: ["a", "b"] }).validate).toBe(true);
    expect(
      // @ts-expect-error tags element type wrong
      validator({ title: "post", tags: [1] }).validate,
    ).toBe(false);
  });

  it("validates nullable arrayOf as a whole", () => {
    const validator = nullable(arrayOf(number()));
    expect(validator(null).validate).toBe(true);
    expect(validator([1, 2]).validate).toBe(true);
    // @ts-expect-error string element not allowed
    expect(validator([1, "2"]).validate).toBe(false);
  });

  it("treats optional inside nested object correctly", () => {
    const validator = object({
      meta: object({
        author: string(),
        editedAt: optional(string()),
      }),
    });
    expect(validator({ meta: { author: "John" } }).validate).toBe(true);
    expect(
      validator({ meta: { author: "John", editedAt: "2025-01-01" } }).validate,
    ).toBe(true);
    expect(
      // @ts-expect-error author is required in nested shape
      validator({ meta: {} }).validate,
    ).toBe(false);
  });
});
