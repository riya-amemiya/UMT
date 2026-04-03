import { parseJson } from "@/Tool/parseJson";

describe("parseJson", () => {
  it("should parse valid JSON string", () => {
    const jsonString = '{"key": "value"}';
    const result = parseJson(jsonString);
    expect(result).toEqual({ key: "value" });
  });

  it("should throw an error for invalid JSON string", () => {
    const invalidJsonString = '{"key": "value"';
    expect(() => parseJson(invalidJsonString)).toThrow(SyntaxError);
  });

  it("should parse JSON string with array", () => {
    const jsonString = '["value1", "value2"]';
    const result = parseJson(jsonString);
    expect(result).toEqual(["value1", "value2"]);
  });

  it("should parse JSON string with nested objects", () => {
    const jsonString = '{"key": {"nestedKey": "nestedValue"}}';
    const result = parseJson(jsonString);
    expect(result).toEqual({ key: { nestedKey: "nestedValue" } });
  });

  it("should parse JSON string with numbers", () => {
    const jsonString = '{"key": 123}';
    const result = parseJson(jsonString);
    expect(result).toEqual({ key: 123 });
  });

  it("should parse JSON string with boolean values", () => {
    const jsonString = '{"key": true}';
    const result = parseJson(jsonString);
    expect(result).toEqual({ key: true });
  });

  it("should strip __proto__ keys to prevent prototype pollution", () => {
    const malicious = '{"__proto__": {"polluted": true}, "safe": 1}';
    const result = parseJson<Record<string, unknown>>(malicious);
    expect(result).toEqual({ safe: 1 });
    expect(result).not.toHaveProperty("__proto__", { polluted: true });
    // Ensure Object.prototype was not polluted
    expect(({} as Record<string, unknown>).polluted).toBeUndefined();
  });

  it("should strip constructor keys to prevent prototype pollution", () => {
    const malicious = '{"constructor": {"prototype": {"polluted": true}}}';
    const result = parseJson<Record<string, unknown>>(malicious);
    // The "constructor" own-property should have been stripped by the reviver;
    // the inherited Object.prototype.constructor should remain a function.
    expect(Object.hasOwn(result, "constructor")).toBe(false);
    expect(typeof result.constructor).toBe("function");
  });

  it("should strip prototype keys in nested objects", () => {
    const malicious = '{"a": {"__proto__": {"polluted": true}, "b": 1}}';
    const result = parseJson<{ a: Record<string, unknown> }>(malicious);
    expect(result.a).toEqual({ b: 1 });
    expect(result.a).not.toHaveProperty("__proto__", { polluted: true });
  });
});
