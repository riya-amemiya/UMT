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
});
