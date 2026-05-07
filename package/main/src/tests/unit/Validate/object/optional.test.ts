import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { optional } from "@/Validate/object/optional";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("optional function", () => {
  it("should allow omitting optional properties in objects", () => {
    const validateObject = object({
      name: string(),
      age: optional(number()),
    });

    const validDataWithoutAge: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
    };

    expect(validateObject(validDataWithoutAge).validate).toBe(true);
  });

  it("should validate defined optional properties normally", () => {
    const validateObject = object({
      name: string(),
      age: optional(number()),
    });

    const validDataWithAge: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      age: 30,
    };

    expect(validateObject(validDataWithAge).validate).toBe(true);
  });

  it("should fail validation for invalid optional property values", () => {
    const validateObject = object({
      name: string(),
      age: optional(number()),
    });

    const invalidData = {
      name: "John Doe",
      age: "not a number",
    };

    // @ts-expect-error
    expect(validateObject(invalidData).validate).toBe(false);
  });

  it("should support multiple optional properties", () => {
    const validateObject = object({
      name: string(),
      age: optional(number()),
      email: optional(string()),
    });

    const validData: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
    };

    expect(validateObject(validData).validate).toBe(true);
  });

  it("should support nested optional objects", () => {
    const validateObject = object({
      name: string(),
      address: optional(
        object({
          street: string(),
          city: string(),
        }),
      ),
    });

    const validDataWithoutAddress: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
    };

    const validDataWithAddress: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      address: {
        street: "123 Main St",
        city: "New York",
      },
    };

    expect(validateObject(validDataWithoutAddress).validate).toBe(true);
    expect(validateObject(validDataWithAddress).validate).toBe(true);
  });

  it("should infer correct types for optional properties", () => {
    const validateObject = object({
      name: string(),
      age: optional(number()),
      email: optional(string()),
    });

    type InferredType = ReturnType<typeof validateObject>["type"];

    const test1: InferredType = {
      name: "test",
    };

    const test2: InferredType = {
      name: "test",
      age: 30,
    };

    const test3: InferredType = {
      name: "test",
      email: "test@example.com",
    };

    const test4: InferredType = {
      name: "test",
      age: 30,
      email: "test@example.com",
    };

    expect(test1).toBeDefined();
    expect(test2).toBeDefined();
    expect(test3).toBeDefined();
    expect(test4).toBeDefined();
  });

  it("should infer optional(string()) as string | undefined via SchemaToInterface", () => {
    const validate = optional(string());
    type Schema = SchemaToInterface<typeof validate>;
    const s: Schema = "hello";
    const u: Schema = undefined;
    expect(validate(s).validate).toBe(true);
    expect(validate(u).validate).toBe(true);
    // @ts-expect-error number is not assignable to string | undefined
    const wrong: Schema = 1;
    expect(wrong).toBe(1);
  });

  it("should infer optional(number()) as number | undefined via SchemaToInterface", () => {
    const validate = optional(number());
    type Schema = SchemaToInterface<typeof validate>;
    const v: Schema = 42;
    const u: Schema = undefined;
    expect(validate(v).validate).toBe(true);
    expect(validate(u).validate).toBe(true);
  });

  it("should infer optional wrapping object as the object shape or undefined via SchemaToInterface", () => {
    const validate = optional(object({ name: string(), age: number() }));
    type Schema = SchemaToInterface<typeof validate>;
    const v: Schema = { name: "John", age: 30 };
    const u: Schema = undefined;
    expect(validate(v).validate).toBe(true);
    expect(validate(u).validate).toBe(true);
  });
});
