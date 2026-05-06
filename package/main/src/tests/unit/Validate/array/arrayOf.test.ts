import { arrayOf } from "@/Validate/array/arrayOf";
import { number } from "@/Validate/number";
import { minValue } from "@/Validate/number/minValue";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

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
    const nullResult = validator(null);
    expect(nullResult.validate).toBe(false);
    expect(nullResult.message).toBe("not an array");
    // @ts-expect-error undefined is not an array
    const undefinedResult = validator(undefined);
    expect(undefinedResult.validate).toBe(false);
    expect(undefinedResult.message).toBe("not an array");
    // @ts-expect-error string is not an array
    const stringResult = validator("not array");
    expect(stringResult.validate).toBe(false);
    expect(stringResult.message).toBe("not an array");
  });

  it("should default the message to empty string when none is provided", () => {
    const validator = arrayOf(object({ name: string() }));
    // @ts-expect-error null is not an array
    const result = validator(null);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("");
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
    const validResult = validator({
      users: [
        { name: "John", age: 30 },
        { name: "Jane", age: 25 },
      ],
    });
    expect(validResult.validate).toBe(true);
    const invalidResult = validator({
      // @ts-expect-error invalid nested age
      users: [{ name: "John", age: "30" }],
    });
    expect(invalidResult.validate).toBe(false);
  });

  it("should preserve the original array in the type field", () => {
    const data = [{ name: "John" }];
    const validator = arrayOf(object({ name: string() }));
    expect(validator(data).type).toBe(data);
  });

  it("should validate arrays of strings using a primitive validator", () => {
    const validator = arrayOf(string());
    expect(validator(["a", "b", "c"]).validate).toBe(true);
    // @ts-expect-error mixed element type
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("should validate arrays of numbers with rule chains", () => {
    const validator = arrayOf(number([minValue(0)]));
    expect(validator([0, 1, 2]).validate).toBe(true);
    expect(validator([0, -1]).validate).toBe(false);
  });

  it("should validate arrays of union elements", () => {
    const validator = arrayOf(union(string(), number()));
    expect(validator(["a", 1, "b"]).validate).toBe(true);
    // @ts-expect-error boolean is not in the union
    expect(validator(["a", true]).validate).toBe(false);
  });

  it("should validate arrays whose elements may be null via nullable", () => {
    const validator = arrayOf(nullable(string()));
    expect(validator(["a", null, "b"]).validate).toBe(true);
    // @ts-expect-error number is not allowed
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("should accept null when the whole array is wrapped by nullable", () => {
    const validator = nullable(arrayOf(string()));
    expect(validator(null).validate).toBe(true);
    expect(validator(["a", "b"]).validate).toBe(true);
    // @ts-expect-error number is not allowed
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("should accept undefined when the whole array is wrapped by optional", () => {
    const validator = optional(arrayOf(string()));
    expect(validator(undefined).validate).toBe(true);
    expect(validator(["a"]).validate).toBe(true);
    // @ts-expect-error number is not allowed
    expect(validator(["a", 1]).validate).toBe(false);
  });

  it("should validate arrays of intersection elements", () => {
    const validator = arrayOf(
      intersection(object({ name: string() }), object({ age: number() })),
    );
    expect(validator([{ name: "John", age: 30 }]).validate).toBe(true);
    // @ts-expect-error missing age
    expect(validator([{ name: "John" }]).validate).toBe(false);
  });

  it("should support nested arrayOf for matrix-like data", () => {
    const validator = arrayOf(arrayOf(number()));
    expect(
      validator([
        [1, 2],
        [3, 4],
      ]).validate,
    ).toBe(true);
    // @ts-expect-error inner element wrong type
    expect(validator([[1, "2"]]).validate).toBe(false);
  });

  it("should stop at the first failing element", () => {
    const calls: number[] = [];
    const trackingValidator = (value: number) => {
      calls.push(value);
      return {
        validate: value >= 0,
        message: value >= 0 ? "" : "negative",
        type: value,
      };
    };
    const validator = arrayOf(trackingValidator);
    const result = validator([1, -1, -2]);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("negative");
    expect(calls).toEqual([1, -1]);
  });

  it("should infer the schema as an array of objects via SchemaToInterface", () => {
    const validator = arrayOf(
      object({
        name: string(),
        age: number(),
      }),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = [
      { name: "John", age: 30 },
      { name: "Jane", age: 25 },
    ];
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error age must be a number, not a string
    const wrongValueType: Schema = [{ name: "John", age: "30" }];
    expect(wrongValueType).toHaveLength(1);
    // @ts-expect-error missing the age property
    const missingProperty: Schema = [{ name: "John" }];
    expect(missingProperty).toHaveLength(1);
    // @ts-expect-error must be an array, not a single object
    const notArray: Schema = { name: "John", age: 30 };
    expect(notArray).toBeDefined();
  });

  it("should infer the schema for an object containing arrayOf", () => {
    const schema = object({
      users: arrayOf(
        object({
          name: string(),
          age: number(),
        }),
      ),
    });
    type Schema = SchemaToInterface<typeof schema>;
    const valid: Schema = {
      users: [
        { name: "John", age: 30 },
        { name: "Jane", age: 25 },
      ],
    };
    expect(schema(valid).validate).toBe(true);
    // @ts-expect-error name must be a string
    const wrong: Schema = { users: [{ name: 1, age: 30 }] };
    expect(wrong.users).toHaveLength(1);
  });

  it("should infer optional properties on inferred schemas", () => {
    const validator = arrayOf(
      object({
        name: string(),
        age: optional(number()),
      }),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const withAge: Schema = [{ name: "John", age: 30 }];
    const withoutAge: Schema = [{ name: "Jane" }];
    expect(validator(withAge).validate).toBe(true);
    expect(validator(withoutAge).validate).toBe(true);
  });

  it("should infer primitive element types via SchemaToInterface", () => {
    const stringArrayValidator = arrayOf(string());
    type StringSchema = SchemaToInterface<typeof stringArrayValidator>;
    const strings: StringSchema = ["a", "b"];
    expect(stringArrayValidator(strings).validate).toBe(true);
    // @ts-expect-error number is not assignable to string
    const wrong: StringSchema = ["a", 1];
    expect(wrong).toHaveLength(2);

    const numberArrayValidator = arrayOf(number([minValue(0)]));
    type NumberSchema = SchemaToInterface<typeof numberArrayValidator>;
    const numbers: NumberSchema = [1, 2];
    expect(numberArrayValidator(numbers).validate).toBe(true);
  });

  it("should infer union element types via SchemaToInterface", () => {
    const validator = arrayOf(union(string(), number()));
    type Schema = SchemaToInterface<typeof validator>;
    const mixed: Schema = ["a", 1, "b", 2];
    expect(validator(mixed).validate).toBe(true);
    // @ts-expect-error boolean is not in the union
    const wrong: Schema = ["a", true];
    expect(wrong).toHaveLength(2);
  });

  it("should infer intersection element types via SchemaToInterface", () => {
    const validator = arrayOf(
      intersection(object({ name: string() }), object({ age: number() })),
    );
    type Schema = SchemaToInterface<typeof validator>;
    const valid: Schema = [{ name: "John", age: 30 }];
    expect(validator(valid).validate).toBe(true);
    // @ts-expect-error missing the age property required by the intersection
    const wrong: Schema = [{ name: "John" }];
    expect(wrong).toHaveLength(1);
  });

  it("should infer nested arrayOf as a 2D array via SchemaToInterface", () => {
    const validator = arrayOf(arrayOf(number()));
    type Schema = SchemaToInterface<typeof validator>;
    const matrix: Schema = [
      [1, 2],
      [3, 4],
    ];
    expect(validator(matrix).validate).toBe(true);
    // @ts-expect-error inner element must be number
    const wrong: Schema = [["a"]];
    expect(wrong).toHaveLength(1);
  });
});
