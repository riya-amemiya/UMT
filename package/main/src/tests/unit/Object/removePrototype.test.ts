import { removePrototype } from "@/Object/removePrototype";

describe("removePrototype", () => {
  it("should remove prototype polluting properties", () => {
    const maliciousPayload =
      '{"__proto__":{"polluted":true},"constructor":{"prototype":{"polluted":true}},"prototype":{"polluted":true},"a":1}';
    const object = JSON.parse(maliciousPayload);

    const result = removePrototype(object);

    expect(result).toEqual({ a: 1 });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(Object.hasOwn(result, "constructor")).toBe(false);
    expect(Object.hasOwn(result, "prototype")).toBe(false);
  });

  it("should preserve other properties", () => {
    const object = {
      a: 1,
      b: "test",
      c: true,
      d: [1, 2, 3],
      e: { nested: true },
    };

    const result = removePrototype(object);

    expect(result).toEqual({
      a: 1,
      b: "test",
      c: true,
      d: [1, 2, 3],
      e: { nested: true },
    });
  });

  it("should not modify the original object", () => {
    const object = JSON.parse('{"__proto__":{"polluted":true},"a":1}');

    const result = removePrototype(object);

    expect(result).toEqual({ a: 1 });
    expect(object.__proto__).toEqual({ polluted: true });
    expect(object.a).toBe(1);
  });

  it("should perform a shallow copy", () => {
    const nested = { b: 2 };
    const object = { a: nested };

    const result = removePrototype(object);

    expect(result.a).toBe(nested);
  });

  it("should preserve type information", () => {
    interface TestObject extends Record<string, unknown> {
      a: number;
      b: string;
      __proto__?: unknown;
    }

    const object: TestObject = { a: 1, b: "test" };
    const result = removePrototype(object);

    expect(result).toEqual({ a: 1, b: "test" });
  });

  it("should return an object with a null prototype", () => {
    const object = { a: 1 };
    const result = removePrototype(object);

    expect(Object.getPrototypeOf(result)).toBeNull();
  });

  it("should not inherit from Object.prototype", () => {
    const object = { a: 1 };
    const result = removePrototype(object);

    expect(result).not.toBeInstanceOf(Object);
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (result as any).hasOwnProperty,
    ).toBeUndefined();
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (result as any).toString,
    ).toBeUndefined();
  });

  it("should handle an empty object", () => {
    const result = removePrototype({});

    expect(result).toEqual({});
    expect(Object.keys(result)).toHaveLength(0);
    expect(Object.getPrototypeOf(result)).toBeNull();
  });

  it("should not copy inherited enumerable properties", () => {
    const parent = { inherited: "value" };
    const child = Object.create(parent);
    child.own = "child";

    const result = removePrototype(child);

    expect(Object.hasOwn(result, "own")).toBe(true);
    expect(Object.hasOwn(result, "inherited")).toBe(false);
  });

  it("should not copy symbol keys", () => {
    const symbol = Symbol("key");
    const object = { a: 1, [symbol]: "value" } as Record<string, unknown>;

    const result = removePrototype(object);

    expect(Object.getOwnPropertySymbols(result)).toHaveLength(0);
    expect((result as Record<symbol, unknown>)[symbol]).toBeUndefined();
  });

  it("should only remove dangerous keys at the top level (shallow)", () => {
    const object = {
      safe: JSON.parse('{"__proto__":{"polluted":true},"value":1}'),
    };

    const result = removePrototype(object);

    expect(Object.hasOwn(result.safe, "__proto__")).toBe(true);
    expect(result.safe).toBe(object.safe);
  });

  it("should demonstrate pollution via mutable assignment and confirm removePrototype prevents it", () => {
    const malicious = JSON.parse('{"__proto__":{"injected":1},"safe":2}');

    // A naive loop assignment uses [[Set]] semantics, which triggers the
    // __proto__ setter and replaces the target's prototype with the
    // attacker-controlled object. Object spread ({...obj}) uses
    // [[DefineOwnProperty]] and would NOT reproduce the attack.
    const vulnerable: { injected?: unknown } = {};
    for (const key of Object.keys(malicious)) {
      (vulnerable as Record<string, unknown>)[key] = malicious[key];
    }
    expect(Object.getPrototypeOf(vulnerable)).toEqual({ injected: 1 });
    expect(vulnerable.injected).toBe(1);

    const safe = removePrototype(malicious) as { injected?: unknown };
    expect(Object.getPrototypeOf(safe)).toBeNull();
    expect(safe.injected).toBeUndefined();
    expect(safe).toEqual({ safe: 2 });
  });

  it("should demonstrate constructor-based pollution and confirm removePrototype prevents it", () => {
    const malicious = JSON.parse('{"constructor":{"polluted":true},"a":1}');

    // Unlike __proto__, `constructor` has no special setter — the spread
    // simply assigns it as a regular own key, so the attacker-controlled
    // object surfaces on the copy.
    const vulnerable = { ...malicious } as {
      constructor: { polluted?: unknown };
    };
    expect(vulnerable.constructor.polluted).toBe(true);

    const safe = removePrototype(malicious) as { constructor?: unknown };
    expect(Object.hasOwn(safe, "constructor")).toBe(false);
    expect(safe.constructor).toBeUndefined();
  });

  it("should not share references between successive calls", () => {
    const object = { a: 1 };

    const first = removePrototype(object);
    const second = removePrototype(object);

    expect(first).not.toBe(second);
  });

  it("should handle an object whose keys contain dangerous names mixed with safe ones", () => {
    const object = JSON.parse(
      '{"__proto__":1,"a":2,"constructor":3,"b":4,"prototype":5,"c":6}',
    );

    const result = removePrototype(object);

    expect(Object.keys(result)).toEqual(["a", "b", "c"]);
    expect(result).toEqual({ a: 2, b: 4, c: 6 });
  });
});
