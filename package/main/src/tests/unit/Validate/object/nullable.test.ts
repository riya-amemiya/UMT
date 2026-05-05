import { nullable } from "@/Validate/object/nullable";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { string } from "@/Validate/string";

describe("nullable function", () => {
  it("should allow null values for nullable properties in objects", () => {
    const validateObject = object({
      name: string(),
      age: nullable(number()),
    });

    const validDataWithNullAge: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      age: null,
    };

    expect(validateObject(validDataWithNullAge).validate).toBe(true);
  });

  it("should validate non-null nullable properties normally", () => {
    const validateObject = object({
      name: string(),
      age: nullable(number()),
    });

    const validDataWithAge: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      age: 30,
    };

    expect(validateObject(validDataWithAge).validate).toBe(true);
  });

  it("should fail validation for invalid nullable property values", () => {
    const validateObject = object({
      name: string(),
      age: nullable(number()),
    });

    const invalidData = {
      name: "John Doe",
      age: "not a number",
    };

    // @ts-expect-error
    expect(validateObject(invalidData).validate).toBe(false);
  });

  it("should support multiple nullable properties", () => {
    const validateObject = object({
      name: string(),
      age: nullable(number()),
      email: nullable(string()),
    });

    const validData: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      age: null,
      email: null,
    };

    expect(validateObject(validData).validate).toBe(true);
  });

  it("should support nested nullable objects", () => {
    const validateObject = object({
      name: string(),
      address: nullable(
        object({
          street: string(),
          city: string(),
        }),
      ),
    });

    const validDataWithNullAddress: ReturnType<typeof validateObject>["type"] =
      {
        name: "John Doe",
        address: null,
      };

    const validDataWithAddress: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      address: {
        street: "123 Main St",
        city: "New York",
      },
    };

    expect(validateObject(validDataWithNullAddress).validate).toBe(true);
    expect(validateObject(validDataWithAddress).validate).toBe(true);
  });

  it("should infer T | null types for nullable properties", () => {
    const validateObject = object({
      name: string(),
      age: nullable(number()),
      email: nullable(string()),
    });

    type InferredType = ReturnType<typeof validateObject>["type"];

    const test1: InferredType = {
      name: "test",
      age: null,
      email: null,
    };

    const test2: InferredType = {
      name: "test",
      age: 30,
      email: null,
    };

    const test3: InferredType = {
      name: "test",
      age: null,
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

  it("should return null type metadata for null values", () => {
    const validateNullable = nullable(number());
    const result = validateNullable(null);
    expect(result.validate).toBe(true);
    expect(result.type).toBe("null");
    expect(result.message).toBe("");
  });

  it("should delegate to wrapped validator for non-null values", () => {
    const validateNullable = nullable(number());
    expect(validateNullable(42).validate).toBe(true);
    // @ts-expect-error
    expect(validateNullable("not a number").validate).toBe(false);
  });
});
