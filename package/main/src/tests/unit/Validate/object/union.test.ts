import { boolean } from "@/Validate/boolean";
import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";

describe("union validation", () => {
  it("should accept values matching the first validator", () => {
    const validate = union(string(), number());
    expect(validate("hello").validate).toBe(true);
    expect(validate("hello").message).toBe("");
  });

  it("should accept values matching the second validator", () => {
    const validate = union(string(), number());
    expect(validate(42).validate).toBe(true);
  });

  it("should reject values matching none of the validators", () => {
    const validate = union(string(), number());
    // @ts-expect-error
    expect(validate(true).validate).toBe(false);
  });

  it("should work with three or more validators", () => {
    const validate = union(string(), number(), boolean());
    expect(validate("hello").validate).toBe(true);
    expect(validate(42).validate).toBe(true);
    expect(validate(true).validate).toBe(true);
    // @ts-expect-error
    expect(validate(null).validate).toBe(false);
  });

  it("should work nested inside object()", () => {
    const validateUser = object({
      id: union(string(), number()),
      name: string(),
    });

    const validWithString: ReturnType<typeof validateUser>["type"] = {
      id: "abc",
      name: "John",
    };
    const validWithNumber: ReturnType<typeof validateUser>["type"] = {
      id: 123,
      name: "John",
    };
    expect(validateUser(validWithString).validate).toBe(true);
    expect(validateUser(validWithNumber).validate).toBe(true);
  });

  it("should return the last failing validator's message", () => {
    const validate = union(
      string([], "must be string"),
      number([], "must be number"),
    );
    // @ts-expect-error
    const result = validate(true);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("must be number");
  });

  it("should infer union type correctly", () => {
    const validate = union(string(), number());
    type InferredType = ReturnType<typeof validate>["type"];

    const strVal: InferredType = "hello" as unknown as InferredType;
    const numVal: InferredType = 42 as unknown as InferredType;
    expect(strVal).toBeDefined();
    expect(numVal).toBeDefined();
  });

  it("should support nested unions", () => {
    const validate = union(union(string(), number()), boolean());
    expect(validate("hello").validate).toBe(true);
    expect(validate(42).validate).toBe(true);
    expect(validate(true).validate).toBe(true);
  });
});
