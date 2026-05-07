import { file } from "@/Validate/file";
import { object } from "@/Validate/object/core";
import { nullable } from "@/Validate/object/nullable";
import { optional } from "@/Validate/object/optional";
import { union } from "@/Validate/object/union";
import { string } from "@/Validate/string";
import type { SchemaToInterface } from "@/Validate/type";

describe("file validation", () => {
  it("accepts File instances", () => {
    const validator = file();
    const fileInstance = new File(["hello"], "hello.txt", {
      type: "text/plain",
    });
    expect(validator(fileInstance).validate).toBe(true);
  });

  it("accepts Blob instances", () => {
    const validator = file();
    const blob = new Blob(["hello"], { type: "text/plain" });
    expect(validator(blob as File).validate).toBe(true);
  });

  it("rejects non-file values", () => {
    const validator = file();
    // @ts-expect-error string is not a File
    expect(validator("text").validate).toBe(false);
    // @ts-expect-error number is not a File
    expect(validator(42).validate).toBe(false);
    // @ts-expect-error plain object is not a File
    expect(validator({}).validate).toBe(false);
  });

  it("returns the configured message on failure", () => {
    const validator = file("must be a file");
    // @ts-expect-error string is not a File
    expect(validator("text").message).toBe("must be a file");
  });

  it("composes inside object()", () => {
    const validator = object({
      attachment: file(),
      name: string(),
    });
    const fileInstance = new File(["hello"], "hello.txt");
    const valid: ReturnType<typeof validator>["type"] = {
      attachment: fileInstance,
      name: "John",
    };
    expect(validator(valid).validate).toBe(true);
  });

  it("composes with union", () => {
    const validator = union(file(), string());
    const fileInstance = new File(["hello"], "hello.txt");
    expect(validator(fileInstance).validate).toBe(true);
    expect(validator("hello").validate).toBe(true);
  });

  it("composes with nullable and optional", () => {
    const fileInstance = new File(["hello"], "hello.txt");
    const nullableFile = nullable(file());
    expect(nullableFile(fileInstance).validate).toBe(true);
    expect(nullableFile(null).validate).toBe(true);

    const optionalFile = optional(file());
    expect(optionalFile(fileInstance).validate).toBe(true);
    expect(optionalFile(undefined).validate).toBe(true);
  });

  it("infers File via SchemaToInterface", () => {
    const validator = file();
    type Schema = SchemaToInterface<typeof validator>;
    const value: Schema = new File(["hello"], "hello.txt");
    expect(validator(value).validate).toBe(true);
  });
});
