import { arrayOf } from "@/Validate/array/arrayOf";
import { map } from "@/Validate/map/core";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { union } from "@/Validate/object/union";
import { set_ } from "@/Validate/set/core";
import { string } from "@/Validate/string";
import { oneOf } from "@/Validate/string/oneOf";
import type { SchemaToInterface } from "@/Validate/type";

/**
 * Integration tests covering compositions of array, map, and set validators
 * with object/union/oneOf validators. Includes deep nesting (3D arrays),
 * arrays of discriminated unions, maps with literal-union keys, and arrays
 * of maps to confirm that ExtractValidatedType-based inference flows through
 * every collection wrapper.
 */
describe("Integration: array, map, and set composition", () => {
  it("validates a 3D array via nested arrayOf", () => {
    const validator = arrayOf(arrayOf(arrayOf(number())));
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = [
      [
        [1, 2],
        [3, 4],
      ],
      [[5, 6, 7]],
    ];
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error innermost element must be number
    expect(validator([[[1, "2"]]]).validate).toBe(false);
  });

  it("validates an array of objects each holding an inner array", () => {
    const validator = arrayOf(
      object({
        name: string(),
        scores: arrayOf(number()),
      }),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = [
      { name: "John", scores: [90, 80] },
      { name: "Jane", scores: [] },
    ];
    expect(validator(valid).validate).toBe(true);
    expect(
      // @ts-expect-error scores element wrong type
      validator([{ name: "John", scores: ["90"] }]).validate,
    ).toBe(false);
  });

  it("validates an array of discriminated union objects", () => {
    const validator = arrayOf(
      union(
        object({ kind: oneOf(["text"]), value: string() }),
        object({ kind: oneOf(["number"]), value: number() }),
      ),
    );
    expect(
      validator([
        { kind: "text", value: "ok" },
        { kind: "number", value: 1 },
      ]).validate,
    ).toBe(true);
    expect(
      // @ts-expect-error value type does not match kind
      validator([{ kind: "text", value: 1 }]).validate,
    ).toBe(false);
  });

  it("validates a Map keyed by a literal union with object values", () => {
    const validator = map(
      oneOf(["en", "ja", "fr"]),
      object({ greeting: string() }),
    );
    const valid = new Map<"en" | "ja" | "fr", { greeting: string }>([
      ["en", { greeting: "Hello" }],
      ["ja", { greeting: "こんにちは" }],
    ]);
    expect(validator(valid).validate).toBe(true);
    const invalid = new Map<"en" | "ja" | "fr", { greeting: string }>([
      ["en", { greeting: 1 } as unknown as { greeting: string }],
    ]);
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates a Map of string keys to arrays of numbers", () => {
    const validator = map(string(), arrayOf(number()));
    const valid = new Map<string, number[]>([
      ["primes", [2, 3, 5]],
      ["evens", [2, 4]],
    ]);
    expect(validator(valid).validate).toBe(true);
    const invalid = new Map<string, number[]>([
      ["mixed", [1, "x" as unknown as number]],
    ]);
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates a Set of literal union elements", () => {
    const validator = set_(oneOf(["red", "green", "blue"]));
    const valid = new Set<"red" | "green" | "blue">(["red", "blue"]);
    expect(validator(valid).validate).toBe(true);
    const invalid = new Set<"red" | "green" | "blue">([
      "red",
      "yellow" as unknown as "red",
    ]);
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates an array containing maps with primitive keys/values", () => {
    const validator = arrayOf(map(string(), number()));
    const valid = [
      new Map<string, number>([["a", 1]]),
      new Map<string, number>([["b", 2]]),
    ];
    expect(validator(valid).validate).toBe(true);
    const invalid = [
      new Map<string, number>([["a", "x" as unknown as number]]),
    ];
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates an array of objects each holding a Set of literal unions", () => {
    const validator = arrayOf(
      object({
        id: number(),
        tags: set_(oneOf(["new", "hot", "sale"])),
      }),
    );
    const valid = [
      { id: 1, tags: new Set<"new" | "hot" | "sale">(["new"]) },
      { id: 2, tags: new Set<"new" | "hot" | "sale">(["hot", "sale"]) },
    ];
    expect(validator(valid).validate).toBe(true);
    const invalid = [
      {
        id: 3,
        tags: new Set<"new" | "hot" | "sale">([
          "new",
          "old" as unknown as "new",
        ]),
      },
    ];
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates a Map whose values are intersections of objects", () => {
    const validator = map(
      string(),
      intersection(object({ name: string() }), object({ count: number() })),
    );
    const valid = new Map<string, { name: string; count: number }>([
      ["a", { name: "alpha", count: 1 }],
    ]);
    expect(validator(valid).validate).toBe(true);
    const invalid = new Map<string, { name: string; count: number }>([
      ["a", { name: "alpha" } as unknown as { name: string; count: number }],
    ]);
    expect(validator(invalid).validate).toBe(false);
  });

  it("validates an arrayOf union of primitives and oneOf literal", () => {
    const validator = arrayOf(union(number(), oneOf(["true", "false"])));
    expect(validator([1, "true", 2, "false"]).validate).toBe(true);
    expect(validator([1, "maybe"]).validate).toBe(false);
  });
});
