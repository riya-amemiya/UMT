import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { nullable } from "@/Validate/object/nullable";
import { union } from "@/Validate/object/union";
import { string, validateEmail } from "@/Validate/string";

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

  it("should work with string inside object the same way as plain string", () => {
    const validateObject = object({
      label: string(),
      description: nullable(string()),
    });

    const validWithString: ReturnType<typeof validateObject>["type"] = {
      label: "title",
      description: "details",
    };
    const validWithNull: ReturnType<typeof validateObject>["type"] = {
      label: "title",
      description: null,
    };
    const invalidNonString = {
      label: "title",
      description: 42,
    };

    expect(validateObject(validWithString).validate).toBe(true);
    expect(validateObject(validWithNull).validate).toBe(true);
    // @ts-expect-error
    expect(validateObject(invalidNonString).validate).toBe(false);
  });

  it("should work with number inside object the same way as plain number", () => {
    const validateObject = object({
      id: number(),
      score: nullable(number()),
    });

    const validWithNumber: ReturnType<typeof validateObject>["type"] = {
      id: 1,
      score: 99,
    };
    const validWithNull: ReturnType<typeof validateObject>["type"] = {
      id: 1,
      score: null,
    };
    const invalidNonNumber = {
      id: 1,
      score: "high",
    };

    expect(validateObject(validWithNumber).validate).toBe(true);
    expect(validateObject(validWithNull).validate).toBe(true);
    // @ts-expect-error
    expect(validateObject(invalidNonNumber).validate).toBe(false);
  });

  it("should work with boolean inside object the same way as plain boolean", () => {
    const validateObject = object({
      id: number(),
      active: nullable(boolean()),
    });

    const validWithBoolean: ReturnType<typeof validateObject>["type"] = {
      id: 1,
      active: true,
    };
    const validWithNull: ReturnType<typeof validateObject>["type"] = {
      id: 1,
      active: null,
    };
    const invalidNonBoolean = {
      id: 1,
      active: "yes",
    };

    expect(validateObject(validWithBoolean).validate).toBe(true);
    expect(validateObject(validWithNull).validate).toBe(true);
    // @ts-expect-error
    expect(validateObject(invalidNonBoolean).validate).toBe(false);
  });

  it("should compose with union to allow string | number | null", () => {
    const validate = union(string(), number(), nullable(string()));
    expect(validate("hello").validate).toBe(true);
    expect(validate(42).validate).toBe(true);
    expect(validate(null).validate).toBe(true);
    // @ts-expect-error
    expect(validate(true).validate).toBe(false);
  });

  it("should wrap a union to allow union members or null", () => {
    const validate = nullable(union(string(), number()));
    expect(validate("hello").validate).toBe(true);
    expect(validate(42).validate).toBe(true);
    expect(validate(null).validate).toBe(true);
    // @ts-expect-error
    expect(validate(true).validate).toBe(false);
  });

  it("should work as a property whose value is a union with null", () => {
    const validateUser = object({
      id: union(string(), number(), nullable(number())),
      name: string(),
    });

    const validWithString: ReturnType<typeof validateUser>["type"] = {
      id: "abc",
      name: "John",
    };
    const validWithNumber: ReturnType<typeof validateUser>["type"] = {
      id: 1,
      name: "John",
    };
    const validWithNull: ReturnType<typeof validateUser>["type"] = {
      id: null,
      name: "John",
    };

    expect(validateUser(validWithString).validate).toBe(true);
    expect(validateUser(validWithNumber).validate).toBe(true);
    expect(validateUser(validWithNull).validate).toBe(true);
  });

  it("should wrap an intersection to allow the intersected shape or null", () => {
    const validate = nullable(
      intersection(object({ name: string() }), object({ age: number() })),
    );

    expect(validate({ name: "John", age: 30 }).validate).toBe(true);
    expect(validate(null).validate).toBe(true);
    // @ts-expect-error
    expect(validate({ name: "John" }).validate).toBe(false);
  });

  it("should work as a property whose value is an intersection or null", () => {
    const validateUser = object({
      name: string(),
      profile: nullable(
        intersection(
          object({ email: string([validateEmail()]) }),
          object({ age: number() }),
        ),
      ),
    });

    const validWithProfile: ReturnType<typeof validateUser>["type"] = {
      name: "John",
      profile: { email: "john@example.com", age: 30 },
    };
    const validWithNullProfile: ReturnType<typeof validateUser>["type"] = {
      name: "John",
      profile: null,
    };

    expect(validateUser(validWithProfile).validate).toBe(true);
    expect(validateUser(validWithNullProfile).validate).toBe(true);
  });

  it("should compose intersection of object validators with nullable properties", () => {
    const validate = intersection(
      object({ name: string() }),
      object({ age: nullable(number()) }),
    );

    expect(validate({ name: "John", age: 30 }).validate).toBe(true);
    expect(validate({ name: "John", age: null }).validate).toBe(true);
    // @ts-expect-error
    expect(validate({ name: "John", age: "30" }).validate).toBe(false);
  });
});
