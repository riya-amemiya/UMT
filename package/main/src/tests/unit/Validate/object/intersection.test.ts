import { number } from "@/Validate/number";
import { object } from "@/Validate/object/core";
import { intersection } from "@/Validate/object/intersection";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";

describe("intersection validation", () => {
  it("should accept values passing all validators", () => {
    const validate = intersection(
      object({ name: string() }),
      object({ age: number() }),
    );

    const validData = { name: "John", age: 30 };
    expect(validate(validData).validate).toBe(true);
    expect(validate(validData).message).toBe("");
  });

  it("should reject values failing any validator", () => {
    const validate = intersection(
      object({ name: string() }),
      object({ age: number() }),
    );

    const invalidData = { name: "John" };
    // @ts-expect-error
    expect(validate(invalidData).validate).toBe(false);
  });

  it("should return the first failing validator's message", () => {
    const validate = intersection(
      object({ name: string() }, "name validation failed"),
      object({ age: number() }, "age validation failed"),
    );

    // @ts-expect-error
    const result = validate("not an object");
    expect(result.validate).toBe(false);
    expect(result.message).toBe("name validation failed");
  });

  it("should work nested inside object()", () => {
    const validateUser = object({
      name: string(),
      profile: intersection(
        object({ email: string() }),
        object({ age: number() }),
      ),
    });

    const validData: ReturnType<typeof validateUser>["type"] = {
      name: "John",
      profile: { email: "john@example.com", age: 30 },
    };
    expect(validateUser(validData).validate).toBe(true);
  });

  it("should infer intersection type correctly", () => {
    const validate = intersection(
      object({ name: string() }),
      object({ age: number() }),
    );
    type InferredType = ReturnType<typeof validate>["type"];

    const data: InferredType = {
      name: "John",
      age: 30,
    } as unknown as InferredType;
    expect(data).toBeDefined();
  });

  it("should support mixed composition with union", () => {
    const validate = union(
      string(),
      intersection(object({ a: number() }), object({ b: string() })),
    );
    expect(validate("hello").validate).toBe(true);
    expect(validate({ a: 1, b: "x" }).validate).toBe(true);
    // @ts-expect-error
    expect(validate(42).validate).toBe(false);
  });

  it("should support nested intersections", () => {
    const validate = intersection(
      intersection(object({ a: string() }), object({ b: number() })),
      object({ c: string() }),
    );

    const validData = { a: "x", b: 1, c: "y" };
    expect(validate(validData).validate).toBe(true);
  });
});
