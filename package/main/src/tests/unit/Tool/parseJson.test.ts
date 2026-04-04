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
    expect(Object.keys(result)).not.toContain("__proto__");
    // Verify Object.prototype was not polluted
    // biome-ignore lint/complexity/useLiteralKeys: TS strict requires bracket access on index signatures
    expect(({} as Record<string, unknown>)["polluted"]).toBeUndefined();
  });

  it("should strip constructor and prototype keys", () => {
    const malicious =
      '{"constructor": {"polluted": true}, "prototype": {"bad": true}, "ok": 2}';
    const result = parseJson<Record<string, unknown>>(malicious);
    expect(result).toEqual({ ok: 2 });
  });

  it("should strip dangerous keys in nested objects", () => {
    const malicious = '{"a": {"__proto__": {"x": 1}, "b": 2}}';
    const result = parseJson<Record<string, unknown>>(malicious);
    expect(result).toEqual({ a: { b: 2 } });
  });
});
