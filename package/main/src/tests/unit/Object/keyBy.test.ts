import { keyBy } from "@/Object/keyBy";
import _ from "lodash";

describe("keyBy", () => {
  it("should create an object using property name as key", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "b2", name: "Bob" },
    ];
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should generate keys using a custom function", () => {
    const input = [
      { dir: "left", code: 97 },
      { dir: "right", code: 100 },
    ];

    const result = {
      a: { dir: "left", code: 97 },
      d: { dir: "right", code: 100 },
    };
    const output = keyBy(input, (o) => String.fromCharCode(o.code));
    const lodashOutput = _.keyBy(input, (o) => String.fromCharCode(o.code));

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should return an empty object for an empty array", () => {
    const result = {};
    const output = keyBy([], "id");
    const lodashOutput = _.keyBy([], "id");
    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should use later values when there are duplicate keys", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "a1", name: "Alex" },
    ];
    const result = {
      a1: { id: "a1", name: "Alex" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should accept an object as input", () => {
    const input = {
      first: { id: "a1", name: "Alice" },
      second: { id: "b2", name: "Bob" },
    };
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });
  it("should act as identity function when iteratee is not specified", () => {
    const input = ["a", "b"];
    const result = {
      a: "a",
      b: "b",
    };
    const output = keyBy(input);
    const lodashOutput = _.keyBy(input);

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should prevent prototype pollution when key resolves to __proto__ (array input)", () => {
    const input = [
      { id: "__proto__", name: "malicious" },
      { id: "safe", name: "ok" },
    ];
    const output = keyBy(input, "id");
    expect(output).toStrictEqual({ safe: { id: "safe", name: "ok" } });
    expect(Object.hasOwn(output, "__proto__")).toBe(false);
  });

  it("should prevent prototype pollution when key resolves to __proto__ (object input)", () => {
    const input = {
      first: { id: "__proto__", name: "malicious" },
      second: { id: "safe", name: "ok" },
    };
    const output = keyBy(input, "id");
    expect(output).toStrictEqual({ safe: { id: "safe", name: "ok" } });
    expect(Object.hasOwn(output, "__proto__")).toBe(false);
  });

  it("should preserve reference equality to the original items", () => {
    const a = { id: "a", payload: { big: "data" } };
    const b = { id: "b", payload: { big: "other" } };

    const output = keyBy([a, b], "id") as {
      a?: typeof a;
      b?: typeof b;
    };

    expect(output.a).toBe(a);
    expect(output.b).toBe(b);
  });

  it("should support symbol keys produced by iteratee", () => {
    const sym = Symbol("k");
    const input = [{ value: 1 }, { value: 2 }];

    const output = keyBy(input, () => sym);

    // Successive items with the same key overwrite earlier ones.
    expect(output[sym]).toEqual({ value: 2 });
  });

  it("should support number keys produced by iteratee and coerce them to string own keys", () => {
    const input = [{ n: 1 }, { n: 2 }];
    const output = keyBy(input, "n");

    expect(Object.keys(output)).toEqual(["1", "2"]);
    expect(output[1]).toEqual({ n: 1 });
    expect(output[2]).toEqual({ n: 2 });
  });

  it("should demonstrate local prototype pollution with a naive grouping, and confirm keyBy never exposes __proto__ as an own key", () => {
    // A naive grouping that assigns into a plain `{}` without filtering
    // triggers the __proto__ setter and locally replaces the result's
    // prototype with the attacker-controlled object.
    const input = [
      { id: "__proto__", value: { injected: "bad" } },
      { id: "safe", value: 1 },
    ];

    const naive: Record<string, unknown> = {};
    for (const item of input) {
      naive[item.id] = item.value;
    }
    expect(Object.getPrototypeOf(naive)).toEqual({ injected: "bad" });
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((naive as any).injected).toBe("bad");

    // keyBy suffers the same local prototype replacement (documented here),
    // but its *own* keys never contain the dangerous `__proto__` entry — so
    // the shape of the returned object is what consumers would expect.
    const safe = keyBy(input, "id") as Record<string, unknown>;
    expect(Object.hasOwn(safe, "__proto__")).toBe(false);
    expect(Object.keys(safe)).toEqual(["safe"]);
  });

  it("should allow keys that collide with Object.prototype names as shadowing own keys", () => {
    const input = [
      { id: "toString", tag: "a" },
      { id: "hasOwnProperty", tag: "b" },
      { id: "constructor", tag: "c" },
    ];

    const output = keyBy(input, "id") as Record<string, unknown>;

    // `toString`, `hasOwnProperty`, and `constructor` are safe as own keys —
    // they simply shadow the inherited methods within the output.
    expect(Object.hasOwn(output, "toString")).toBe(true);
    expect(Object.hasOwn(output, "hasOwnProperty")).toBe(true);
    expect(Object.hasOwn(output, "constructor")).toBe(true);
    expect(output.toString).toEqual({ id: "toString", tag: "a" });
    expect(output.hasOwnProperty).toEqual({
      id: "hasOwnProperty",
      tag: "b",
    });
    expect(output.constructor).toEqual({ id: "constructor", tag: "c" });
  });

  it("should return an object whose prototype is Object.prototype when no __proto__ key is produced", () => {
    const output = keyBy([{ id: "a" }], "id");
    expect(Object.getPrototypeOf(output)).toBe(Object.prototype);
  });

  it("should treat `prototype` and `constructor` as regular own keys (keyBy only guards __proto__)", () => {
    // Documents the library's scope: only __proto__ is blocked (by the JS
    // language itself via the setter). `prototype` and `constructor` become
    // regular own keys and may collide with usage expectations — callers who
    // accept untrusted iteratee outputs should sanitize further.
    const output = keyBy([{ id: "prototype" }, { id: "constructor" }], "id");

    expect(Object.hasOwn(output, "prototype")).toBe(true);
    expect(Object.hasOwn(output, "constructor")).toBe(true);
  });
});
