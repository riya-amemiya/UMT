import { arrayOf } from "@/Validate/array/arrayOf";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { optional } from "@/Validate/object/optional";
import { string } from "@/Validate/string";

describe("arrayOf validation", () => {
  it("should validate an empty array", () => {
    const validator = arrayOf(object({ name: string() }));
    expect(validator([]).validate).toBe(true);
  });

  it("should validate an array of valid objects", () => {
    const validator = arrayOf(
      object({
        name: string(),
        age: number(),
      }),
    );
    const result = validator([
      { name: "John", age: 30 },
      { name: "Jane", age: 25 },
    ]);
    expect(result.validate).toBe(true);
    expect(result.message).toBe("");
  });

  it("should fail when an element does not match the object validator", () => {
    const validator = arrayOf(
      object({
        name: string(),
        age: number(),
      }),
    );
    const result = validator([
      { name: "John", age: 30 },
      // @ts-expect-error invalid age type
      { name: "Jane", age: "twenty-five" },
    ]);
    expect(result.validate).toBe(false);
  });

  it("should fail when the value is not an array", () => {
    const validator = arrayOf(object({ name: string() }), "not an array");
    // @ts-expect-error null is not an array
    const result = validator(null);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("not an array");
  });

  it("should propagate the element validation message", () => {
    const message = "name must be a string";
    const validator = arrayOf(
      object({
        name: string([], message),
      }),
    );
    // @ts-expect-error name has wrong type
    const result = validator([{ name: 1 }]);
    expect(result.validate).toBe(false);
    expect(result.message).toBe(message);
  });

  it("should validate arrays of objects with optional properties", () => {
    const validator = arrayOf(
      object({
        name: string(),
        age: optional(number()),
      }),
    );
    const result = validator([{ name: "John" }, { name: "Jane", age: 25 }]);
    expect(result.validate).toBe(true);
  });

  it("should support nesting arrayOf inside object", () => {
    const validator = object({
      users: arrayOf(
        object({
          name: string(),
          age: number(),
        }),
      ),
    });
    const result = validator({
      users: [
        { name: "John", age: 30 },
        { name: "Jane", age: 25 },
      ],
    });
    expect(result.validate).toBe(true);
  });

  it("should preserve the original array in the return type field", () => {
    const data = [{ name: "John" }];
    const validator = arrayOf(object({ name: string() }));
    expect(validator(data).type).toBe(data);
  });
});
