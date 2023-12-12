import { object } from "@/Validate/object/core";
import { string } from "@/Validate/string";
import { number } from "@/Validate/number";
import { array } from "@/Validate";

describe("object validation", () => {
  it("should validate an object with string and number types", () => {
    const validateObject = object(
      {
        name: string([], "string"),
        age: number([], "number"),
        array: array<string | number>(
          {
            string: string([], "string"),
            number: number([], "number"),
          },
          "array",
        ),
      },
      "object",
    );

    const validData: ReturnType<typeof validateObject>["type"] = {
      name: "John Doe",
      age: 30,
      array: ["John Doe", 30],
    };

    const invalidData = {
      name: "John Doe",
      age: "thirty",
    };
    expect(validateObject(validData).message).toBe("");
    expect(validateObject(validData).validate).toBe(true);
    // @ts-ignore
    expect(validateObject(invalidData).validate).toBe(false);
  });

  it("should return a custom message on number validation failure", () => {
    const customMessage = "Invalid object structure";
    const validateObject = object({
      name: string([]),
      age: number([], customMessage),
    });

    const invalidData = {
      name: "John Doe",
      age: "thirty",
    };
    // @ts-ignore
    const result = validateObject(invalidData);
    expect(result.validate).toBe(false);
    expect(result.message).toBe(customMessage);
  });

  it("should return a custom message on validation failure", () => {
    const customMessage = "Invalid object structure";
    const validateObject = object(
      {
        name: string([]),
        age: number([]),
      },
      customMessage,
    );

    const invalidData = "John Doe";
    // @ts-ignore
    const result = validateObject(invalidData);
    expect(result.validate).toBe(false);
    expect(result.message).toBe(customMessage);
  });

  it("should return an empty message on validation failure", () => {
    const validateObject = object({
      name: string([]),
      age: number([]),
    });

    const invalidData = "John Doe";
    // @ts-ignore
    const result = validateObject(invalidData);
    expect(result.validate).toBe(false);
    expect(result.message).toBe("");
  });
});
