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
});
