import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { optional } from "@/Validate/object/optional";
import { string } from "@/Validate/string";

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
});
