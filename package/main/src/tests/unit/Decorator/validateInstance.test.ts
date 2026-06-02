import { IsArray } from "@/Decorator/IsArray";
import { IsBoolean } from "@/Decorator/IsBoolean";
import { IsNumber } from "@/Decorator/IsNumber";
import { IsString } from "@/Decorator/IsString";
import { LengthBetween } from "@/Decorator/LengthBetween";
import { Max } from "@/Decorator/Max";
import { Min } from "@/Decorator/Min";
import { Nullable } from "@/Decorator/Nullable";
import { Optional } from "@/Decorator/Optional";
import { Schema } from "@/Decorator/Schema";
import { validateInstance } from "@/Decorator/validateInstance";
import type { StandardSchemaV1 } from "@/Validate/standardSchema";

describe("validateInstance", () => {
  it("returns success when every field is valid", () => {
    class User {
      @IsString name = "alice";
      @IsNumber age = 20;
    }

    const result = validateInstance(new User());

    expect(result.type).toBe("success");
    if (result.type === "success") {
      expect(result.value).toBeInstanceOf(User);
    }
  });

  it("reports the failing field by path and message", () => {
    class User {
      @IsNumber age = "oops" as unknown as number;
    }

    const result = validateInstance(new User());

    expect(result.type).toBe("error");
    if (result.type === "error") {
      expect(result.error).toEqual([
        { path: "age", message: "age must be a number" },
      ]);
    }
  });

  it("rejects numeric strings for IsNumber", () => {
    class Box {
      @IsNumber value = "1" as unknown as number;
    }

    expect(validateInstance(new Box()).type).toBe("error");
  });

  it("validates IsString, IsArray, and IsBoolean", () => {
    class Bag {
      @IsString label = 1 as unknown as string;
      @IsArray items = "no" as unknown as unknown[];
      @IsBoolean active = "yes" as unknown as boolean;
    }

    const result = validateInstance(new Bag());

    expect(result.type).toBe("error");
    if (result.type === "error") {
      expect(result.error.map((issue) => issue.path).sort()).toEqual([
        "active",
        "items",
        "label",
      ]);
    }
  });

  it("enforces Min and Max bounds", () => {
    class Range {
      @Min(0) low = -1;
      @Max(10) high = 11;
    }

    const result = validateInstance(new Range());

    expect(result.type).toBe("error");
    if (result.type === "error") {
      expect(result.error).toHaveLength(2);
    }
  });

  it("accepts values exactly on the Min and Max boundary", () => {
    class Range {
      @Min(0) low = 0;
      @Max(10) high = 10;
    }

    expect(validateInstance(new Range()).type).toBe("success");
  });

  it("enforces LengthBetween on strings and arrays", () => {
    class Form {
      @LengthBetween(1, 3) tags = [1, 2, 3, 4];
      @LengthBetween(2, 5) code = "x";
    }

    const result = validateInstance(new Form());

    expect(result.type).toBe("error");
    if (result.type === "error") {
      expect(result.error).toHaveLength(2);
    }
  });

  it("validates against a Standard Schema validator", () => {
    const evenNumber: StandardSchemaV1<unknown, number> = {
      "~standard": {
        version: 1,
        vendor: "test",
        validate: (value) =>
          typeof value === "number" && value % 2 === 0
            ? { value }
            : { issues: [{ message: "not even" }] },
      },
    };

    class Counter {
      @Schema(evenNumber) total = 3;
    }

    expect(validateInstance(new Counter()).type).toBe("error");
  });

  it("treats asynchronous Standard Schema validators as a failure", () => {
    const asyncSchema: StandardSchemaV1<unknown, number> = {
      "~standard": {
        version: 1,
        vendor: "test",
        validate: (value) => Promise.resolve({ value: value as number }),
      },
    };

    class AsyncCounter {
      @Schema(asyncSchema) total = 1;
    }

    expect(validateInstance(new AsyncCounter()).type).toBe("error");
  });

  it("skips rules for undefined fields marked Optional", () => {
    class Profile {
      @Optional
      @IsString
      nickname?: string;
    }

    expect(validateInstance(new Profile()).type).toBe("success");
  });

  it("still validates Optional fields that hold a value", () => {
    class Profile {
      @Optional
      @IsString
      nickname: string = 1 as unknown as string;
    }

    expect(validateInstance(new Profile()).type).toBe("error");
  });

  it("skips rules for null fields marked Nullable", () => {
    class Profile {
      @Nullable
      @IsString
      bio: string | null = null;
    }

    expect(validateInstance(new Profile()).type).toBe("success");
  });

  it("collects rules from base classes", () => {
    class Base {
      @IsString id = 0 as unknown as string;
    }
    class Derived extends Base {
      @IsNumber count = "x" as unknown as number;
    }

    const result = validateInstance(new Derived());

    expect(result.type).toBe("error");
    if (result.type === "error") {
      expect(result.error.map((issue) => issue.path).sort()).toEqual([
        "count",
        "id",
      ]);
    }
  });
});
