import { parseJson } from "@/Tool/parseJson";
import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object";

describe("Integration test for 'parseJson' and 'Validate' functions", () => {
  it("should parse JSON string with number values", () => {
    const jsonString = '{"key": 123}';
    const schema = object({
      key: number(),
    });
    const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
    const isValid = schema(result).validate;
    expect(isValid).toEqual(true);
  });

  it("should parse JSON string with number values", () => {
    const jsonString = '{"key": true}';
    const schema = object({
      key: boolean(),
    });
    const result = parseJson<ReturnType<typeof schema>["type"]>(jsonString);
    const isValid = schema(result).validate;
    expect(isValid).toEqual(true);
  });
});
