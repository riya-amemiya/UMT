import { detectMode } from "@/String/formatString/detectMode";

describe("detectMode function", () => {
  describe("named mode detection", () => {
    test("should detect named mode with object data only", () => {
      const result = detectMode({ name: "Alice", age: 30 }, undefined, []);

      expect(result.data).toEqual({ name: "Alice", age: 30 });
      expect(result.options).toEqual({});
    });

    test("should detect named mode with object data and options", () => {
      const data = { name: "Bob", score: 95 };
      const options = { formatters: { custom: () => "test" } };
      const result = detectMode(data, options, []);

      expect(result.data).toBe(data);
      expect(result.options).toBe(options);
    });

    test("should handle empty object as named mode", () => {
      const result = detectMode({}, undefined, []);

      expect(result.data).toEqual({});
      expect(result.options).toEqual({});
    });

    test("should handle complex nested objects", () => {
      const complexData = {
        user: { name: "Alice", profile: { age: 25 } },
        items: ["item1", "item2"],
      };
      const result = detectMode(complexData, undefined, []);

      expect(result.data).toBe(complexData);
      expect(result.options).toEqual({});
    });
  });

  describe("indexed mode detection", () => {
    test("should detect indexed mode with multiple primitive values", () => {
      const result = detectMode("first", "second", ["third", "fourth"]);

      expect(result.data).toEqual(["first", "second", "third", "fourth"]);
      expect(result.options).toEqual({});
    });

    test("should detect indexed mode with single value", () => {
      const result = detectMode("single", undefined, []);

      expect(result.data).toEqual(["single"]);
      expect(result.options).toEqual({});
    });

    test("should handle mixed types in indexed mode", () => {
      const result = detectMode("string", 42, [true, null, { key: "value" }]);

      expect(result.data).toEqual(["string", 42, true, null, { key: "value" }]);
      expect(result.options).toEqual({});
    });

    test("should handle undefined as first value", () => {
      const result = detectMode(undefined, "second", ["third"]);

      expect(result.data).toEqual(["second", "third"]);
      expect(result.options).toEqual({});
    });
  });

  describe("object type differentiation", () => {
    test("should treat arrays as indexed mode", () => {
      const result = detectMode(["array", "values"], "second", []);

      expect(result.data).toEqual([["array", "values"], "second"]);
      expect(result.options).toEqual({});
    });

    test("should treat Date objects as indexed mode", () => {
      const date = new Date("2024-01-01");
      const result = detectMode(date, "second", []);

      expect(result.data).toEqual([date, "second"]);
      expect(result.options).toEqual({});
    });

    test("should treat null as indexed mode", () => {
      const result = detectMode(null, "second", []);

      expect(result.data).toEqual([null, "second"]);
      expect(result.options).toEqual({});
    });

    test("should differentiate between data object and options object", () => {
      const dataObject = { name: "Alice" };
      const optionsObject = {
        formatters: { upper: (s: string) => s.toUpperCase() },
      };

      const result = detectMode(dataObject, optionsObject, []);

      expect(result.data).toBe(dataObject);
      expect(result.options).toBe(optionsObject);
    });
  });

  describe("edge cases", () => {
    test("should handle all undefined values", () => {
      const result = detectMode(undefined, undefined, []);

      expect(result.data).toEqual([]);
      expect(result.options).toEqual({});
    });

    test("should handle object with additional values as indexed mode", () => {
      const result = detectMode({ name: "Alice" }, "extra", ["more"]);

      expect(result.data).toEqual([{ name: "Alice" }, "extra", "more"]);
      expect(result.options).toEqual({});
    });

    test("should handle non-options object as second parameter", () => {
      const result = detectMode({ name: "Alice" }, { notFormatters: true }, []);

      expect(result.data).toEqual([{ name: "Alice" }, { notFormatters: true }]);
      expect(result.options).toEqual({});
    });

    test("should handle primitive values as first argument", () => {
      expect(detectMode("string", undefined, []).data).toEqual(["string"]);
      expect(detectMode(42, undefined, []).data).toEqual([42]);
      expect(detectMode(true, undefined, []).data).toEqual([true]);
      expect(detectMode(false, undefined, []).data).toEqual([false]);
    });
  });

  describe("options object validation", () => {
    test("should recognize valid options object", () => {
      const options = {
        formatters: {
          custom: (value: any) => String(value),
          upper: (s: string) => s.toUpperCase(),
        },
      };
      const result = detectMode({ name: "test" }, options, []);

      expect(result.data).toEqual({ name: "test" });
      expect(result.options).toBe(options);
    });

    test("should reject object without formatters property", () => {
      const notOptions = { someOtherProperty: "value" };
      const result = detectMode({ name: "test" }, notOptions, []);

      expect(result.data).toEqual([{ name: "test" }, notOptions]);
      expect(result.options).toEqual({});
    });

    test("should reject array as options", () => {
      const result = detectMode({ name: "test" }, ["not", "options"], []);

      expect(result.data).toEqual([{ name: "test" }, ["not", "options"]]);
      expect(result.options).toEqual({});
    });

    test("should reject Date as options", () => {
      const date = new Date();
      const result = detectMode({ name: "test" }, date, []);

      expect(result.data).toEqual([{ name: "test" }, date]);
      expect(result.options).toEqual({});
    });
  });

  describe("complex scenarios", () => {
    test("should handle deeply nested data structures", () => {
      const complexData = {
        level1: {
          level2: {
            level3: {
              value: "deep",
            },
          },
        },
        array: [1, 2, { nested: "object" }],
      };

      const result = detectMode(complexData, undefined, []);

      expect(result.data).toBe(complexData);
      expect(result.options).toEqual({});
    });

    test("should preserve object references", () => {
      const originalObject = { shared: "reference" };
      const result = detectMode(originalObject, undefined, []);

      expect(result.data).toBe(originalObject);
      expect(result.data === originalObject).toBe(true);
    });

    test("should handle mixed scenarios correctly", () => {
      // Test that when first arg is object but has extra values, it goes to indexed mode
      const obj = { key: "value" };
      const result = detectMode(obj, "string", [42, true]);

      expect(result.data).toEqual([obj, "string", 42, true]);
      expect(result.options).toEqual({});
    });
  });
});
